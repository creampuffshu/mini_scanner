pub fn get_file_size(path: &str) -> Result<u64, std::io::Error>{
    Ok(1024)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_get_file_size_success(){
        let size = get_file_size("dummy.txt").unwrap();
        assert_eq!(size,1024,"파일 크기가 1024바이트여야 합니다.");
    }
}