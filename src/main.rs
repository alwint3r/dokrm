use std::{
    env,
    process::{Command, Stdio},
};

fn main() {
    let image_name = get_image_name(env::args()).unwrap();

    let proc = Command::new("docker")
        .args(["images"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let result = proc.wait_with_output().unwrap();

    let stdout = String::from_utf8_lossy(result.stdout.as_slice());
    let splat = stdout.split('\n');

    let images = splat
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let image_tags: Vec<&str> = line.split(' ').filter(|line| !line.is_empty()).collect();
            format!("{}:{}", image_tags[0], image_tags[1])
        })
        .filter(|image| image.len() > 0);

    let selected_images: Vec<String> = images
        .filter(|image| image.contains(&image_name))
        .collect();

    for image in selected_images {
        println!("Removing {}", image);

        let rmi = Command::new("docker")
            .arg("rmi")
            .arg(&image)
            .stdout(Stdio::piped())
            .spawn();

        if let Ok(child) = rmi {
            if let Ok(_) = child.wait_with_output() {
                println!("Removed {}", image);
            }
        }
    }
}

fn get_image_name(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
    args.next();

    match args.next() {
        Some(image_name) => Ok(image_name),
        None => Err("Didn't catch the image name"),
    }
}
