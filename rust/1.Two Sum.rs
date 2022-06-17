impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        
        // 맵 데이터 세팅
        for (pos, e) in nums.clone()
            .into_iter()
            .enumerate()
        {
            let found = map.get_mut(&e);
            
            if found.is_some() {
                found.unwrap().push(pos);
            } 
            else {
                map.insert(e, vec![pos]);
            }
        }
        
        // 순회해서 인덱스값 반환
        for (pos, e) in nums
            .into_iter()
            .enumerate() 
        {
            let need = target - e;
            let found = map.get(&need);
            
            if let Some(found) = found {
                let pos_list: Vec<_> = found
                    .into_iter()
                    .filter(
                        |pos2|
                        &pos!=*pos2
                    )
                    .collect();
                if pos_list.len() ==0 {
                    continue;
                } 
                else {
                    return vec![pos as i32, *pos_list[0] as i32];
                }
            } else {
                continue;
            }
        }
        
        vec![]
    }
}
