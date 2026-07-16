//! Bounded Rust source compilation aligned with Hutool's compiler workflow.

use std::{
    any::type_name,
    collections::{BTreeMap, HashMap},
    error::Error,
    ffi::OsString,
    fmt::{self, Display},
    fs, io,
    path::{Component, Path, PathBuf},
    process::Command,
};

use tempfile::TempDir;

use crate::format_template;

/// Default maximum size of one in-memory source unit.
pub const DEFAULT_MAX_SOURCE_BYTES: usize = 4 * 1024 * 1024;

/// Compilation failure with diagnostics and an optional source chain.
#[derive(Debug)]
pub struct CompilerException {
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl CompilerException {
    /// Creates an error from a message.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// Creates an error and preserves its source.
    pub fn from_error<E: Error + Send + Sync + 'static>(source: E) -> Self {
        let kind = type_name::<E>()
            .rsplit("::")
            .next()
            .unwrap_or(type_name::<E>());
        Self {
            message: format!("{kind}: {source}"),
            source: Some(Box::new(source)),
        }
    }

    /// Formats a message using `{}` placeholders.
    #[must_use]
    pub fn formatted(template: &str, params: &[&dyn Display]) -> Self {
        Self::new(format_template(template, params))
    }

    /// Creates an error from a message and source.
    pub fn with_source<E: Error + Send + Sync + 'static>(
        message: impl Into<String>,
        source: E,
    ) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Formats a message and preserves its source.
    pub fn formatted_with_source<E: Error + Send + Sync + 'static>(
        source: E,
        template: &str,
        params: &[&dyn Display],
    ) -> Self {
        Self::with_source(format_template(template, params), source)
    }
}

impl Display for CompilerException {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for CompilerException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn Error + 'static))
    }
}

impl From<io::Error> for CompilerException {
    fn from(source: io::Error) -> Self {
        Self::from_error(source)
    }
}

/// One bounded UTF-8 Rust source unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFileObject {
    name: PathBuf,
    source: String,
}

impl SourceFileObject {
    /// Creates a validated bounded source unit.
    pub fn new(name: &str, source: &str, max_bytes: usize) -> Result<Self, CompilerException> {
        let name = PathBuf::from(name);
        validate_source_name(&name)?;
        if source.len() > max_bytes {
            return Err(CompilerException::formatted(
                "source {} exceeds {} bytes",
                &[&name.display(), &max_bytes],
            ));
        }
        Ok(Self {
            name,
            source: source.to_owned(),
        })
    }

    /// Reads a bounded UTF-8 source file.
    pub fn from_path(path: &Path, max_bytes: usize) -> Result<Self, CompilerException> {
        let name = path
            .file_name()
            .ok_or_else(|| CompilerException::new("source path has no file name"))?;
        if fs::metadata(path)?.len() > max_bytes as u64 {
            return Err(CompilerException::formatted(
                "source {} exceeds {} bytes",
                &[&path.display(), &max_bytes],
            ));
        }
        let source = fs::read_to_string(path)?;
        Self::new(&name.to_string_lossy(), &source, max_bytes)
    }

    /// Returns the relative source name.
    #[must_use]
    pub fn name(&self) -> &Path {
        &self.name
    }

    /// Returns the source text.
    #[must_use]
    pub fn char_content(&self) -> &str {
        &self.source
    }
}

/// One in-memory compiler artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassFileObject {
    name: String,
    bytes: Vec<u8>,
}

impl ClassFileObject {
    /// Creates an empty named artifact.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            bytes: Vec::new(),
        }
    }
    /// Returns the artifact name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[must_use]
    /// Opens the produced bytes for reading.
    pub fn open_input(&self) -> &[u8] {
        &self.bytes
    }
    /// Opens the artifact buffer for writing.
    pub fn open_output(&mut self) -> &mut Vec<u8> {
        &mut self.bytes
    }
}

/// In-memory artifact registry.
#[derive(Debug, Default)]
pub struct ClassFileManager {
    artifacts: HashMap<String, ClassFileObject>,
}

impl ClassFileManager {
    /// Creates or gets an output artifact.
    pub fn output(&mut self, name: &str) -> &mut ClassFileObject {
        let name = name.to_owned();
        self.artifacts
            .entry(name.clone())
            .or_insert_with(|| ClassFileObject::new(&name))
    }
    /// Gets a produced artifact.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&ClassFileObject> {
        self.artifacts.get(name)
    }
}

/// File-name and source-loading helpers.
pub struct SourceFileObjectUtil;

impl SourceFileObjectUtil {
    /// Tests for a `.rs` suffix.
    #[must_use]
    pub fn is_source_file(name: &str) -> bool {
        extension_eq(Path::new(name), "rs")
    }
    /// Tests for a supported Rust library suffix.
    #[must_use]
    pub fn is_library_file(name: &str) -> bool {
        ["rlib", "a", "so", "dylib", "dll"]
            .iter()
            .any(|ext| extension_eq(Path::new(name), ext))
    }
    /// Loads one supported source path.
    pub fn get_source_file_objects(
        path: &Path,
        max_bytes: usize,
    ) -> Result<Vec<SourceFileObject>, CompilerException> {
        if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(Self::is_source_file)
        {
            Ok(vec![SourceFileObject::from_path(path, max_bytes)?])
        } else {
            Ok(Vec::new())
        }
    }
}

/// Joins compiler diagnostics with line separators.
#[must_use]
pub fn diagnostic_messages(messages: &[impl AsRef<str>]) -> String {
    messages
        .iter()
        .map(AsRef::as_ref)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Result of a successful `rustc` invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileOutput {
    /// Newly produced paths.
    pub artifacts: Vec<PathBuf>,
    /// Compiler standard output.
    pub stdout: String,
    /// Compiler standard error diagnostics.
    pub diagnostics: String,
}

/// Bounded builder for compiling Rust source through an explicit `rustc` process.
#[derive(Debug)]
pub struct RustSourceCompiler {
    rustc: PathBuf,
    staging_parent: Option<PathBuf>,
    sources: BTreeMap<PathBuf, SourceFileObject>,
    libraries: Vec<PathBuf>,
    options: Vec<OsString>,
    max_source_bytes: usize,
}

impl Default for RustSourceCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl RustSourceCompiler {
    /// Creates a compiler using `rustc` from `PATH`.
    /// Selects a compiler executable.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rustc: "rustc".into(),
            staging_parent: None,
            sources: BTreeMap::new(),
            libraries: Vec::new(),
            options: Vec::new(),
            max_source_bytes: DEFAULT_MAX_SOURCE_BYTES,
        }
    }
    /// Selects a compiler executable.
    #[must_use]
    pub fn rustc(mut self, executable: PathBuf) -> Self {
        self.rustc = executable;
        self
    }
    /// Sets the maximum size per source unit.
    #[must_use]
    pub fn max_source_bytes(mut self, max: usize) -> Self {
        self.max_source_bytes = max;
        self
    }
    /// Selects the parent directory used for temporary compiler inputs.
    #[must_use]
    pub fn staging_parent(mut self, parent: PathBuf) -> Self {
        self.staging_parent = Some(parent);
        self
    }
    /// Adds or replaces one in-memory source.
    pub fn add_source(&mut self, name: &str, source: &str) -> Result<&mut Self, CompilerException> {
        let unit = SourceFileObject::new(name, source, self.max_source_bytes)?;
        self.sources.insert(unit.name.clone(), unit);
        Ok(self)
    }
    /// Adds multiple in-memory sources.
    pub fn add_sources(
        &mut self,
        sources: Vec<(&str, &str)>,
    ) -> Result<&mut Self, CompilerException> {
        for (name, source) in sources {
            self.add_source(name, source)?;
        }
        Ok(self)
    }
    /// Adds one source file from disk.
    pub fn add_source_path(&mut self, path: &Path) -> Result<&mut Self, CompilerException> {
        let unit = SourceFileObject::from_path(path, self.max_source_bytes)?;
        self.sources.insert(unit.name.clone(), unit);
        Ok(self)
    }
    /// Adds a dependency search path.
    pub fn add_library(&mut self, path: PathBuf) -> &mut Self {
        self.libraries.push(path);
        self
    }
    /// Adds an explicit `rustc` option.
    pub fn add_option(&mut self, option: OsString) -> &mut Self {
        self.options.push(option);
        self
    }

    /// Compiles the configured crate into an explicit output directory.
    ///
    /// # Panics
    ///
    /// Panics only if the internal validated relative source-name invariant is
    /// violated; public constructors prevent that state.
    pub fn compile_to(&self, output_dir: &Path) -> Result<CompileOutput, CompilerException> {
        let root = self.crate_root()?;
        let staging = create_staging(self.staging_parent.as_deref())?;
        write_sources(staging.path(), self.sources.values())?;
        fs::create_dir_all(output_dir)?;
        let mut command = Command::new(&self.rustc);
        command
            .arg(staging.path().join(root))
            .arg("--crate-type=lib")
            .arg("--error-format=short")
            .arg("--out-dir")
            .arg(output_dir);
        for library in &self.libraries {
            command
                .arg("-L")
                .arg(format!("dependency={}", library.display()));
        }
        command.args(&self.options);
        let output = command.output().map_err(CompilerException::from_error)?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let diagnostics = String::from_utf8_lossy(&output.stderr).into_owned();
        if !output.status.success() {
            return Err(CompilerException::new(diagnostics));
        }
        let artifacts = directory_entries(output_dir)?;
        Ok(CompileOutput {
            artifacts,
            stdout,
            diagnostics,
        })
    }

    fn crate_root(&self) -> Result<&Path, CompilerException> {
        for root in [Path::new("lib.rs"), Path::new("main.rs")] {
            if self.sources.contains_key(root) {
                return Ok(root);
            }
        }
        if self.sources.len() == 1 {
            return Ok(self
                .sources
                .keys()
                .next()
                .expect("length checked")
                .as_path());
        }
        Err(CompilerException::new(
            "multiple source units require lib.rs or main.rs",
        ))
    }
}

fn validate_source_name(name: &Path) -> Result<(), CompilerException> {
    let valid = !name.as_os_str().is_empty()
        && name
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(SourceFileObjectUtil::is_source_file)
        && name
            .components()
            .all(|component| matches!(component, Component::Normal(_)));
    if valid {
        Ok(())
    } else {
        Err(CompilerException::formatted(
            "invalid relative Rust source name: {}",
            &[&name.display()],
        ))
    }
}

fn extension_eq(path: &Path, expected: &str) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case(expected))
}

fn create_staging(parent: Option<&Path>) -> Result<TempDir, CompilerException> {
    match parent {
        Some(parent) => TempDir::new_in(parent),
        None => TempDir::new(),
    }
    .map_err(CompilerException::from_error)
}

fn write_sources<'a>(
    staging: &Path,
    units: impl IntoIterator<Item = &'a SourceFileObject>,
) -> Result<(), CompilerException> {
    for unit in units {
        let path = staging.join(unit.name());
        let parent = path
            .parent()
            .expect("validated relative source joined to a staging directory has a parent");
        fs::create_dir_all(parent)?;
        fs::write(path, unit.char_content())?;
    }
    Ok(())
}

fn directory_entries(path: &Path) -> Result<Vec<PathBuf>, CompilerException> {
    let entries = fs::read_dir(path)?.map(|entry| entry.map(|entry| entry.path()));
    collect_entries(entries)
}

fn collect_entries(
    entries: impl IntoIterator<Item = io::Result<PathBuf>>,
) -> Result<Vec<PathBuf>, CompilerException> {
    let mut entries = entries.into_iter().collect::<Result<Vec<_>, _>>()?;
    entries.sort();
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_objects_enforce_names_limits_and_manage_artifacts() {
        assert!(SourceFileObjectUtil::is_source_file("LIB.RS"));
        assert!(!SourceFileObjectUtil::is_source_file("lib.java"));
        assert!(SourceFileObjectUtil::is_library_file("libdemo.RLIB"));
        assert!(!SourceFileObjectUtil::is_library_file("libdemo.txt"));
        let unit = SourceFileObject::new("src/value.rs", "pub const V: u8 = 1;", 64).unwrap();
        assert_eq!(unit.name(), Path::new("src/value.rs"));
        assert!(unit.char_content().contains("const V"));
        assert!(SourceFileObject::new("../escape.rs", "", 1).is_err());
        assert!(SourceFileObject::new("", "", 1).is_err());
        assert!(SourceFileObject::new("/absolute.rs", "", 1).is_err());
        assert!(SourceFileObject::new("value.txt", "", 1).is_err());
        assert!(SourceFileObject::new("value.rs", "too large", 2).is_err());
        let mut manager = ClassFileManager::default();
        manager
            .output("demo")
            .open_output()
            .extend_from_slice(b"abc");
        assert_eq!(manager.output("demo").open_input(), b"abc");
        assert_eq!(manager.get("demo").map(ClassFileObject::name), Some("demo"));
        assert_eq!(
            manager.get("demo").map(ClassFileObject::open_input),
            Some(b"abc".as_slice())
        );
        assert!(manager.get("missing").is_none());
        assert_eq!(diagnostic_messages(&["one", "two"]), "one\ntwo");
    }

    #[test]
    fn source_loading_and_exception_constructors_are_bounded() {
        let temp = tempfile::tempdir().unwrap();
        let source = temp.path().join("demo.rs");
        fs::write(&source, "pub fn demo() {}").unwrap();
        assert_eq!(
            SourceFileObjectUtil::get_source_file_objects(&source, 64)
                .unwrap()
                .len(),
            1
        );
        assert!(
            SourceFileObjectUtil::get_source_file_objects(temp.path(), 64)
                .unwrap()
                .is_empty()
        );
        assert!(SourceFileObject::from_path(&source, 2).is_err());
        assert!(SourceFileObject::from_path(&temp.path().join("missing.rs"), 64).is_err());
        assert!(SourceFileObject::from_path(Path::new(""), 64).is_err());
        let binary = temp.path().join("binary.rs");
        fs::write(&binary, [0xff]).unwrap();
        assert!(SourceFileObject::from_path(&binary, 64).is_err());
        assert!(
            SourceFileObjectUtil::get_source_file_objects(&temp.path().join("missing.rs"), 64)
                .is_err()
        );
        let plain = CompilerException::new("plain");
        assert_eq!(plain.to_string(), "plain");
        assert!(plain.source().is_none());
        let from = CompilerException::from_error(io::Error::other("io"));
        assert!(from.to_string().contains("io") && from.source().is_some());
        assert_eq!(
            CompilerException::formatted("bad {}", &[&"code"]).to_string(),
            "bad code"
        );
        assert!(
            CompilerException::with_source("bad", io::Error::other("source"))
                .source()
                .is_some()
        );
        let both =
            CompilerException::formatted_with_source(io::Error::other("source"), "bad {}", &[&2]);
        assert_eq!(both.to_string(), "bad 2");
        assert!(both.source().is_some());
        let converted: CompilerException = io::Error::other("converted").into();
        assert!(converted.to_string().contains("converted"));

        let invalid_parent = temp.path().join("parent-file");
        fs::write(&invalid_parent, "file").unwrap();
        assert!(create_staging(Some(&invalid_parent)).is_err());
        assert!(create_staging(Some(temp.path())).is_ok());

        let flat = SourceFileObject::new("node.rs", "", 1).unwrap();
        let nested = SourceFileObject::new("node.rs/child.rs", "", 1).unwrap();
        let ascending = tempfile::tempdir().unwrap();
        assert!(write_sources(ascending.path(), [&flat, &nested]).is_err());
        let descending = tempfile::tempdir().unwrap();
        assert!(write_sources(descending.path(), [&nested, &flat]).is_err());
        let other = SourceFileObject::new("other.rs", "", 1).unwrap();
        let valid = tempfile::tempdir().unwrap();
        assert!(write_sources(valid.path(), [&flat, &other]).is_ok());

        assert_eq!(
            collect_entries(vec![Ok(PathBuf::from("b")), Ok(PathBuf::from("a"))]).unwrap(),
            [PathBuf::from("a"), PathBuf::from("b")]
        );
        assert!(
            collect_entries(vec![
                Err(io::Error::other("entry")),
                Ok(PathBuf::from("unused")),
            ])
            .is_err()
        );
    }

    #[test]
    fn rust_source_compiler_builds_and_reports_diagnostics() {
        let temp = tempfile::tempdir().unwrap();
        let extra = temp.path().join("extra.rs");
        fs::write(&extra, "pub const EXTRA: u8 = 2;").unwrap();
        let mut compiler = RustSourceCompiler::default().max_source_bytes(256);
        assert!(compiler.add_source("bad.txt", "").is_err());
        assert!(
            compiler
                .add_sources(vec![("ok.rs", ""), ("bad.txt", "")])
                .is_err()
        );
        assert!(
            compiler
                .add_source_path(&temp.path().join("missing.rs"))
                .is_err()
        );
        compiler
            .add_sources(vec![(
                "lib.rs",
                "mod extra; pub fn answer() -> u8 { 40 + extra::EXTRA }",
            )])
            .unwrap();
        compiler.add_source_path(&extra).unwrap();
        compiler
            .add_library(temp.path().to_path_buf())
            .add_option("--edition=2021".into());
        let output = compiler.compile_to(&temp.path().join("out")).unwrap();
        assert!(!output.artifacts.is_empty());
        assert!(output.stdout.is_empty() && output.diagnostics.is_empty());

        let mut main_root = RustSourceCompiler::new();
        main_root
            .add_source("main.rs", "pub fn value() -> u8 { 1 }")
            .unwrap();
        assert!(
            !main_root
                .compile_to(&temp.path().join("main-out"))
                .unwrap()
                .artifacts
                .is_empty()
        );
        let mut invalid = RustSourceCompiler::new().rustc("rustc".into());
        invalid.add_source("broken.rs", "not valid rust").unwrap();
        assert!(
            invalid
                .compile_to(&temp.path().join("bad"))
                .unwrap_err()
                .to_string()
                .contains("error")
        );
        let mut ambiguous = RustSourceCompiler::new();
        ambiguous.add_source("a.rs", "").unwrap();
        ambiguous.add_source("b.rs", "").unwrap();
        assert!(
            ambiguous
                .compile_to(&temp.path().join("ambiguous"))
                .is_err()
        );
        let mut missing = RustSourceCompiler::new().rustc(temp.path().join("missing-rustc"));
        missing.add_source("lib.rs", "").unwrap();
        assert!(missing.compile_to(&temp.path().join("missing")).is_err());

        let output_file = temp.path().join("not-a-directory");
        fs::write(&output_file, "occupied").unwrap();
        assert!(missing.compile_to(&output_file).is_err());
        assert!(directory_entries(temp.path().join("absent").as_path()).is_err());

        let mut invalid_staging = RustSourceCompiler::new().staging_parent(output_file.clone());
        invalid_staging.add_source("lib.rs", "").unwrap();
        assert!(
            invalid_staging
                .compile_to(&temp.path().join("never"))
                .is_err()
        );

        let mut colliding_sources = RustSourceCompiler::new();
        colliding_sources.add_source("lib.rs", "").unwrap();
        colliding_sources.add_source("lib.rs/child.rs", "").unwrap();
        assert!(
            colliding_sources
                .compile_to(&temp.path().join("collision"))
                .is_err()
        );
    }

    #[cfg(unix)]
    #[test]
    fn compiler_reports_artifact_directory_disappearing() {
        use std::os::unix::fs::PermissionsExt;

        let temp = tempfile::tempdir().unwrap();
        let fake = temp.path().join("fake-rustc.sh");
        fs::write(&fake, "#!/bin/sh\nfor last; do :; done\nrm -rf \"$last\"\n").unwrap();
        let mut permissions = fs::metadata(&fake).unwrap().permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&fake, permissions).unwrap();

        let mut compiler = RustSourceCompiler::new().rustc(fake);
        compiler.add_source("lib.rs", "").unwrap();
        assert!(compiler.compile_to(&temp.path().join("vanishing")).is_err());
    }
}
