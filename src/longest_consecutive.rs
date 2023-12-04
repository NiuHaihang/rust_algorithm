
pub mod longest{
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {

    }

    struct UF{
        pre: Vec<i32>,
        size: Vec<i32>,
    }

    impl UF{
        fn new(n:i32)->UF{
            let mut pre:Vec<i32> = Vec::new();
            let mut size:Vec<i32>=vec![1,n];

            for i in 1..n{
                pre.push(i);
            }
            UF{
                pre:pre,
                size:size,
            }
        }

        fn find(&self, i:i32)->i32{
            if self.pre[i] == i{
                return i;
            }
            self.pre[i] = find(self.pre[i]);
            self.pre[i]
        }

        fn union(&self,i:i32,j:i32){
            let x = self.find(i);
            let y = self.find(j);

            if x < y{
                self.pre[x] = y;
                self.size[y] += self.size[x];
                return
            }
            self.pre[y] = x;
            self.size[x] += self.size[y];
        }
    }

}