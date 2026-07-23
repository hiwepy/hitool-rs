//! 对齐: `cn.hutool.core.math.Arrangement`
//! 来源: hutool-core/src/main/java/cn/hutool/core/math/Arrangement.java

/// 排列 A(n, m) —— 对齐 Java `Arrangement`。
#[derive(Debug, Clone)]
pub struct Arrangement {
    datas: Vec<String>,
}

impl Arrangement {
    /// 对齐 Java: `Arrangement(String[] datas)`
    pub fn new(datas: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            datas: datas.into_iter().map(Into::into).collect(),
        }
    }

    /// 对齐 Java: `Arrangement.count(int n)` → A(n, n)
    pub fn count_n(n: i32) -> i64 {
        Self::count(n, n)
    }

    /// 对齐 Java: `Arrangement.count(int n, int m)` → A(n, m) = n!/(n-m)!
    pub fn count(n: i32, m: i32) -> i64 {
        if m < 0 || n < 0 || m > n {
            panic!("n >= 0 && m >= 0 && m <= n required");
        }
        if m == 0 {
            return 1;
        }
        let mut result: i64 = 1;
        for i in 0..m {
            let next = result.checked_mul((n - i) as i64);
            match next {
                Some(v) => result = v,
                None => panic!("Overflow computing A({},{})", n, m),
            }
        }
        result
    }

    /// 对齐 Java: `Arrangement.countAll(int n)` → Σ A(n,i) for i=1..n
    pub fn count_all(n: i32) -> i64 {
        let mut total = 0i64;
        for i in 1..=n {
            total += Self::count(n, i);
        }
        total
    }

    /// 对齐 Java: `select()` 全排列（m = n）
    pub fn select_full(&self) -> Vec<Vec<String>> {
        self.select(self.datas.len() as i32)
    }

    /// 对齐 Java: `select(int m)` —— 不重复排列（DFS）
    pub fn select(&self, m: i32) -> Vec<Vec<String>> {
        let n = self.datas.len() as i32;
        if m < 0 || m > n {
            return Vec::new();
        }
        if m == 0 {
            return vec![Vec::new()];
        }
        let mut result = Vec::new();
        let mut visited = vec![false; self.datas.len()];
        let mut buffer = vec![String::new(); m as usize];
        self.dfs(&mut buffer, 0, &mut visited, &mut result);
        result
    }

    /// 对齐 Java: `selectAll()` —— m=1..n 全部排列
    pub fn select_all(&self) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        for m in 1..=self.datas.len() as i32 {
            result.extend(self.select(m));
        }
        result
    }

    /// 对齐 Java: `iterate(int m)`
    pub fn iterate(&self, m: i32) -> Vec<Vec<String>> {
        self.select(m)
    }

    fn dfs(
        &self,
        buffer: &mut [String],
        depth: usize,
        visited: &mut [bool],
        result: &mut Vec<Vec<String>>,
    ) {
        if depth == buffer.len() {
            result.push(buffer.to_vec());
            return;
        }
        for i in 0..self.datas.len() {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            buffer[depth] = self.datas[i].clone();
            self.dfs(buffer, depth + 1, visited, result);
            visited[i] = false;
        }
    }
}
