pub fn dp_rec_mc(amount: u32) -> u32 {
   // 确保硬币面额按升序排列
   let coins = [1, 2, 5, 10, 20, 50, 100];
    
   // 创建 DP 数组
   let mut dp = vec![std::u32::MAX; (amount + 1) as usize];
   
   // 基本情况：0 金额需要 0 个硬币
   dp[0] = 0;
   
   // 构建解决方案
   for i in 1..=amount {
       for &coin in coins.iter() {
           if coin <= i {
               let sub_res = dp[(i - coin) as usize];
               if sub_res != std::u32::MAX {
                   dp[i as usize] = dp[i as usize].min(sub_res + 1);
               }
           }
       }
   }
   
   dp[amount as usize]
}
