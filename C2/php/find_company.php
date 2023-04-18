<?php
    if( exec('grep '.escapeshellarg($_GET['companyName']).' ./companies.txt')) {
        // print "Company found";
        echo "<p>Company found</p>";
    } else {
        // print "Company not found";
        echo "<p>Company not found</p>";
    }
?>