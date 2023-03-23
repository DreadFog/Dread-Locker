<?php
    if( exec('grep '.escapeshellarg($_GET['companyName']).' ./companies.txt')) {
        // print "Company found";
        echo "Company found";
    } else {
        // print "Company not found";
        echo "Company not found";
    }
?>