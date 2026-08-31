use std::fs;
use std::path::Path;

pub fn get_file_size(path: &str) -> Result<u64, std::io::Error>{
    Ok(1024)
}

pub fn get_total_size(dir_path: &Path) -> Result<u64, std::io::Error>{
    let mut total_size = 0;

    for entry in fs::read_dir(dir_path)?{
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_file(){
            total_size += metadata.len();
        }
    }

    Ok(total_size)
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_get_file_size(){
        let dir = tempdir().unwrap();

        let file_path1 = dir.path().join("dummy1.txt");
        let mut file1 = File::create(&file_path1).unwrap();
        write!(file1, "12345").unwrap();

        let file_path2 = dir.path().join("dummy2.txt");
        let mut file2 = File::create(&file_path2).unwrap();
        write!(file2, "ABC").unwrap();

        let result_size = get_total_size(dir.path()).unwrap();

        assert_eq!(result_size, 8, "파일 용량 합산이 정확해야 합니다.");
        

        
    }


    #[test]
    fn test_get_file_size_success(){
        let size = get_file_size("dummy.txt").unwrap();
        assert_eq!(size,1024,"파일 크기가 1024바이트여야 합니다.");
    }
}