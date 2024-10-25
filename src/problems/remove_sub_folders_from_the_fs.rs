use std::collections::HashMap;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_subfolders(folders: Vec<String>) -> Vec<String> {
        let mut fs = FolderTree {
            root: FolderNode {
                is_directory: false,
                children: HashMap::new(),
            },
        };
        for folder in folders.iter() {
            fs.add_folder(folder.to_string());
        }

        folders
            .into_iter()
            .filter(|folder| !fs.is_subfolder(folder.to_string()))
            .collect()
    }
}

struct FolderNode {
    pub is_directory: bool,
    pub children: HashMap<String, FolderNode>,
}

struct FolderTree {
    pub root: FolderNode,
}

impl FolderTree {
    pub fn add_folder(&mut self, folder: String) {
        let mut current = &mut self.root;
        for folder_name in folder.split('/') {
            current = current
                .children
                .entry(folder_name.to_string())
                .or_insert(FolderNode {
                    is_directory: false,
                    children: HashMap::new(),
                });
        }
        current.is_directory = true;
    }

    pub fn is_subfolder(&self, folder: String) -> bool {
        let mut current = &self.root;
        let folder_names = folder.split('/');
        let count = folder_names.clone().count();
        for folder_name in folder_names.take(count - 1) {
            if let Some(child) = current.children.get(folder_name) {
                if child.is_directory {
                    return true;
                }
                current = child;
            } else {
                return false;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let folders = vec![
            "/a".to_string(),
            "/a/b".to_string(),
            "/c/d".to_string(),
            "/c/d/e".to_string(),
            "/c/f".to_string(),
        ];
        let res = Solution::remove_subfolders(folders);
        let expected = vec!["/a".to_string(), "/c/d".to_string(), "/c/f".to_string()];
        assert_eq!(res, expected);
    }

    #[test]
    fn test2() {
        let folders = vec!["/a".to_string(), "/a/b/c".to_string(), "/a/b/d".to_string()];
        let res = Solution::remove_subfolders(folders);
        let expected = vec!["/a".to_string()];
        assert_eq!(res, expected);
    }

    #[test]
    fn test3() {
        let folders = vec![
            "/a/b/c".to_string(),
            "/a/b/ca".to_string(),
            "/a/b/d".to_string(),
        ];
        let res = Solution::remove_subfolders(folders);
        let expected = vec![
            "/a/b/c".to_string(),
            "/a/b/ca".to_string(),
            "/a/b/d".to_string(),
        ];
        assert_eq!(res, expected);
    }
}
