pub mod longest {
    use std::cmp::max;
    use std::collections::HashMap;
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut mp: HashMap<i32, i32> = HashMap::new();

        let mut uf = build_uf(n.try_into().unwrap());

        for (i, num) in nums.iter().enumerate() {
            match mp.get(num) {
                Some(&i32) => continue,
                None => {}
            }
            match mp.get(&(num - 1)) {
                Some(&i32) => uf.union(i as i32, uf.find(mp.get(&(num - 1)).copied().unwrap())),
                None => {}
            }
            match mp.get(&(num + 1)) {
                Some(&i32) => uf.union(i as i32, uf.find(mp.get(&(num + 1)).copied().unwrap())),
                None => {}
            }
            mp.insert(*num, i as i32);
        }

        let mut res = 0;
        for (i, size) in uf.size.iter().enumerate() {
            res = max(res, *size)
        }
        return res;
    }

    struct UF {
        pre: Vec<i32>,
        size: Vec<i32>,
    }
    fn build_uf(n: i32) -> UF {
        let mut pre: Vec<i32> = Vec::new();
        let mut size: Vec<i32> = vec![1, n];

        for i in 1..n {
            pre.push(i);
        }
        UF {
            pre: pre,
            size: size,
        }
    }
    impl UF {
        fn find(&mut self, i: i32) -> i32 {
            if self.pre[i as usize] == i {
                return i;
            }
            self.pre[i as usize] = self.find(self.pre[i as usize]);
            self.pre[i as usize]
        }

        fn union(&mut self, i: i32, j: i32) {
            let x = self.find(i);
            let y = self.find(j);

            if x < y {
                self.pre[x as usize] = y;
                self.size[y as usize] += self.size[x as usize];
                return;
            }
            self.pre[y as usize] = x;
            self.size[x as usize] += self.size[y as usize];
        }
    }
}
