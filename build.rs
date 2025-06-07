fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("your_icon.ico")?;
        res.compile()?;
    }

    Ok(())
}
