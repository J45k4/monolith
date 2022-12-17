


use std::{cmp::min, fmt::Debug};

#[derive(Debug, PartialEq)]
pub enum EditOperation<T> {
    InsertFirst(T),
    InsertAt(usize, T),
    RemoveAt(usize),
    ReplaceAt(usize, T),
}

pub fn get_minimum_edits<T: PartialEq + Clone>(s: &Vec<T>, t: &Vec<T>) -> Vec<EditOperation<T>> {
    let m = s.len();
    let n = t.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Initialize the base cases
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    // Fill in the DP table
    for i in 1..=m {
        for j in 1..=n {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = min(dp[i - 1][j], dp[i][j - 1]) + 1;
            }
        }
    }

    // Initialize the list of edit operations
    let mut edits = Vec::new();

    // Backtrack to generate the list of edit operations
    let mut i = m;
    let mut j = n;
    while i > 0 || j > 0 {
        if i == 0 {
            // Insert t[j - 1]
            edits.push(EditOperation::InsertFirst(t[j - 1].clone()));
            j -= 1;

            continue;
        } else if j == 0 {
            // Delete s[i - 1]
            edits.push(EditOperation::RemoveAt(i - 1));
            i -= 1;

            continue;
        } else if s[i - 1] == t[j - 1] {
            // No operation required
            i -= 1;
            j -= 1;

            continue;
        } 
        
        let top = dp[i - 1][j];
        let left = dp[i][j - 1];
        let diag = dp[i - 1][j - 1];

        if diag < top && diag < left {
            edits.push(EditOperation::ReplaceAt(i - 1, t[j - 1].clone()));
            i -= 1;
            j -= 1;
        } else if top < left {
            edits.push(EditOperation::RemoveAt(i - 1));
            i -= 1;
        } else {
            edits.push(EditOperation::InsertAt(i - 1, t[j - 1].clone()));
            j -= 1;
        }
    }

    // Reverse the list of edit operations to get the correct order
    edits.reverse();

    edits
}
    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_front() {
        let a = vec![1, 2, 3];
        let b = vec![0, 1, 2, 3];

        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::InsertFirst(0)]);
    }
    
    #[test]
    fn test_insert_back() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 3, 4];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::InsertAt(2, 4)]);
    }
    
    #[test]
    fn test_insert_middle() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 4, 3];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::InsertAt(1, 4)]);
    }
    
    #[test]
    fn test_remove_first() {
        let a = vec![1, 2, 3];
        let b = vec![2, 3];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::RemoveAt(0)]);
    }
    
    #[test]
    fn test_remove_last() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::RemoveAt(2)]);
    }
    
    #[test]
    fn test_remove_middle() {
        let a = vec![1, 2, 3];
        let b = vec![1, 3];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::RemoveAt(1)]);
    }
    
    #[test]
    fn test_replace_first() {
        let a = vec![1, 2, 3];
        let b = vec![4, 2, 3];

        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::ReplaceAt(0, 4)]);
    }
    
    #[test]
    fn test_replace_last() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 4];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::ReplaceAt(2, 4)]);
    }
    
    #[test]
    fn test_replace_middle() {
        let a = vec![1, 2, 3];
        let b = vec![1, 4, 3];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::ReplaceAt(1, 4)]);
    }
    
    #[test]
    fn test_replace_all() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::ReplaceAt(0, 4), EditOperation::ReplaceAt(1, 5), EditOperation::ReplaceAt(2, 6)]);
    }

    #[test]
    fn test_move_from_back_to_front() {
        let a = vec![1, 2, 3];
        let b = vec![3, 1, 2];

        let edits = get_minimum_edits(&a, &b);
    
        assert_eq!(edits, vec![EditOperation::InsertFirst(3), EditOperation::RemoveAt(2)]);
    }

    #[test]
    fn test_string() {
        let old = "sitting";
        let new = "kitten";

        let edits = get_minimum_edits(&old.chars().collect::<Vec<_>>(), &new.chars().collect::<Vec<_>>());

        assert_eq!(edits, 
            vec![
                EditOperation::ReplaceAt(0, 'k'),
                EditOperation::ReplaceAt(4, 'e'),
                EditOperation::RemoveAt(6),
            ]
        );
    }
}

