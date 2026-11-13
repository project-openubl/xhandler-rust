use std::io::{Cursor, Read, Write};

use anyhow::anyhow;
use base64::engine::general_purpose;
use base64::Engine;
use zip::result::ZipResult;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::encoding::decode_xml;

pub fn create_zip(content: &[u8], file_name_inside_zip: &str) -> ZipResult<Vec<u8>> {
    let mut data = Vec::new();

    {
        let buff = Cursor::new(&mut data);
        let mut zip = ZipWriter::new(buff);

        let file_options = SimpleFileOptions::default();
        zip.start_file(file_name_inside_zip, file_options)?;
        zip.write_all(content)?;
        zip.finish()?;
    }

    Ok(data)
}

pub fn read_cdr_from_base64_zip(base64: &str) -> anyhow::Result<String> {
    let zip_buf = general_purpose::STANDARD.decode(base64)?;
    read_cdr_from_zip(&zip_buf)
}

/// Returns the decoded CDR XML from a SUNAT zip: the first `.xml` entry, or the
/// first file if no entry has that extension. SUNAT zips usually include a
/// `dummy/` directory entry, which is skipped.
pub fn read_cdr_from_zip(zip_buf: &[u8]) -> anyhow::Result<String> {
    let mut archive = ZipArchive::new(Cursor::new(zip_buf))?;

    let file_names: Vec<String> = archive
        .file_names()
        .filter(|name| !name.ends_with('/'))
        .map(String::from)
        .collect();
    let cdr_name = file_names
        .iter()
        .find(|name| name.to_lowercase().ends_with(".xml"))
        .or(file_names.first())
        .ok_or_else(|| anyhow!("The CDR zip has no files"))?;

    let mut buffer = Vec::new();
    archive.by_name(cdr_name)?.read_to_end(&mut buffer)?;
    decode_xml(&buffer)
}

#[cfg(test)]
mod tests {
    use crate::zip_manager::read_cdr_from_base64_zip;

    #[test]
    fn read_base64_zip() {
        let cdr = "UEsDBBQAAgAIAEwklVcAAAAAAgAAAAAAAAAGAAAAZHVtbXkvAwBQSwMEFAACAAgATCSVVwwGkuIxBAAAGQ0AABsAAABSLTEyMzQ1Njc4OTEyLTAxLUYwMDEtMS54bWy1V2FP2zwQ/r5fEZUPk6Y3OElbWKPQqVDGsgFitDC0byY52uhN7WA7peXXv+e4SdMStHbSq/LBuXvuubvHZ1sEXxaz1JqDkAlnJy330GlZwCIeJ2xy0robf7U/t770PwRU+IMsS5OIKgTegsw4k2BhMJMnrVwwn1OZSJ/RGUhfZhAlTyuwnz+mvoymMKP+QsZ+yOY8icD2Wibcp2JPhoZK1mywUHvSnfHZjLPzhQKmVcBPpASm5Jo0eoz+ivQU4VEjIf07wsFkImBCFTSRxrgVU6Uyn5CXl5fDl/YhFxPiOY5DnB5BTCyTyUGJlpxmFd4kkofo0vYiUC8IsDmkPANSJcHkVRgsZKoKsDZLm7LYVgn2UiUp+5Q5o+rdPjMQeb3ZkUY39eqWxIv3enXJw9XlqKAqscgCi6yhaHTkKRU2egVIvfmy1Q9wgvy708tqIGQ55g0+Y6nNDsOV6gejZIId5KI6IjvsCx4zHQZxyJ54/4NlBWeUcYY6pclrodUVqCmPrUE64SJR09m7EriOpsW+IjtyO+zgF6L1AGkNW6TgrircmdTplLXaMy7gQEhqyyntut6K8haeQODtAdbdbajlQiOax4Iy+cTFTBpD3fTHtBsSlcMY27Ks3qTek3QXgZCQbFceDJMJSLWnYqjIQV2niueepjn0H16PRPsBEvXt+ntGw3m4hMvrQXZ+OoB0zKYv98+/w2V4/hS/dm/uur+PB/C8OLvoEX41ypxzGf5KLn9cPF9E38/vczn8OVVikRz9PDkJSD2L3h9SbRCOGtmctfpEmIhPNyKZ4+mz/oWl9fEUFL3Bo4rXGQj10WJcWXn2ydDUooIfsCw4g4eu0xtSRc1KR5kzj8zXeA3EVrQ2rfhNQmSo8W8HF2yhlDmIEYiEpnWLJt6fvhZbcBne63z2CGJ/to3oeoKyXLJWhlRqrXXEdfOdQt5ePm9Msh/gW6VN9+ZND4d979AJyBtrgTvLpeKz1e2CRreEbjsKtAYcO20XJ/mo2z72DLTy6iaHeos8x+3Zrmd7nbHj+MXfClpB1hFjfC76DbDCXsDKN37F7bULbtdgN5wbcEPc8dsd3+tuglfcNPJrqq960ZbR3fVgXOuuAnKxvKFCLY2tWIYxbk71mlU0qEEbf16v210TkfejSoeZQh1QrGqVGA/ZQpL3isPDnyiaVg0OlKLRdFZMkvbrkRGMpus7wUzObdg/2NJA20yihiDyp2RkW2f9CSwG8f9ISRoT3EIEyXznnK7X7nSPjj/3XG/nnA0phjzKtQrl4JW1VF/FUK60xBRf8dWwq8le2zcG+4zHONibE13YCtQQZCSSrKjrklpfaYSyU4thHYJbJsE/1pRaMom5RSPIFI2pYavHlj3VC1+3szEzW4VXUjXBjU5JlqB9x704wvNe/fbZjY0spHk/SPN/Nv3/AFBLAQIAABQAAgAIAEwklVcAAAAAAgAAAAAAAAAGAAAAAAAAAAAAAAAAAAAAAABkdW1teS9QSwECAAAUAAIACABMJJVXDAaS4jEEAAAZDQAAGwAAAAAAAAABAAAAAAAmAAAAUi0xMjM0NTY3ODkxMi0wMS1GMDAxLTEueG1sUEsFBgAAAAACAAIAfQAAAJAEAAAAAA==";

        let result = read_cdr_from_base64_zip(cdr).expect("Could not extract cdr file");
        assert!(result.contains("ApplicationResponse"));
    }

    #[test]
    fn extract_xml_entry_before_other_files() {
        use std::io::{Cursor, Write};
        use zip::write::SimpleFileOptions;
        use zip::ZipWriter;

        let mut data = Vec::new();
        {
            let mut zip = ZipWriter::new(Cursor::new(&mut data));
            zip.add_directory("dummy/", SimpleFileOptions::default())
                .unwrap();
            zip.start_file("readme.txt", SimpleFileOptions::default())
                .unwrap();
            zip.write_all(b"not the cdr").unwrap();
            zip.start_file("R-123.XML", SimpleFileOptions::default())
                .unwrap();
            zip.write_all(b"<cdr/>").unwrap();
            zip.finish().unwrap();
        }

        let result = super::read_cdr_from_zip(&data).unwrap();
        assert_eq!(result, "<cdr/>");
    }
}
