<?php

if ($_SERVER['REQUEST_METHOD'] == 'GET') {
    echo '<!DOCTYPE html><form action="http://localhost/uploadToC2.php" method="post" enctype="multipart/form-data">
    Select file to upload:
    <input type="file" name="uploaded_file">
    <input type="submit" value="Upload File" name="submit">
    </form>';
    exit();
}

if ($_SERVER['REQUEST_METHOD'] == 'POST' && isset($_FILES['uploaded_file'])) {
if (!isset($_COOKIE['token']) || $_COOKIE['token'] !== 'FiT4bNJ4U6R==XwkOomwZrytTasZA7ujpUFFFa19Mc9gb3BERrzrxLUFKcp1Wn5?p5d3Hhoo/B0m8y6q=zPmojQ9dV6jT9bIYf57/uA7-f/LnZw7NtC9dCjr!l8dm-Pv') {
    echo "Unauthorized request";
    exit();
}
$file_size = $_FILES['uploaded_file']['size'];
$file_tmp = $_FILES['uploaded_file']['tmp_name'];
$file_type = $_FILES['uploaded_file']['type'];

// Get the file from the uploaded file
$file_ext = explode('.', $_FILES['uploaded_file']['name']);
$file_ext = strtolower(end($file_ext));
if ($file_ext == 'php') {
    echo "Error: PHP files are not allowed";
    exit();
}
$file_name = bin2hex(random_bytes(10)) . "." . $file_ext;
// Check if the uploaded file is a GZIP file using magic bytes
$finfo = finfo_open(FILEINFO_MIME_TYPE);
$mime_type = finfo_file($finfo, $file_tmp);
if ($mime_type !== 'application/gzip') {
    echo "Error: The uploaded file is not a GZIP file";
    exit();
}
finfo_close($finfo);

// Move the uploaded file to a desired location
move_uploaded_file($file_tmp, "/var/www/html/upload/" . $file_name);
echo "File uploaded successfully";
} else {
echo "No file uploaded";
}