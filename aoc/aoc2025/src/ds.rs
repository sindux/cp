#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n]
        }
    }

    pub fn find(&mut self, mut i: usize) -> usize {
        let mut root = i;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        while self.parent[i] != root {
            (self.parent[i], i) = (root, self.parent[i])
        }
        root
    }

    pub fn union(&mut self, i: usize, j: usize) -> usize {
        let iroot = self.find(i);
        let jroot = self.find(j);
        if iroot != jroot {
            let (smaller, bigger) = if self.size[iroot] < self.size[jroot] {
                (iroot, jroot)
            } else {
                (jroot, iroot)
            };
            self.parent[smaller] = bigger;
            self.size[bigger] += self.size[smaller];
            bigger
        } else {
            iroot
        }
    }

    pub fn size(&mut self, i: usize) -> usize {
        let root = self.find(i);
        self.size[root]
    }
}