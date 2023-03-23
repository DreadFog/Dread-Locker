# Creating a Container
`veracrypt --text --create vctest.vc --size 200M --password MySuperSecurePassword1! --volume-type normal --encryption AES --hash sha-512 --filesystem exfat --pim 0 --keyfiles ""`

# Mounting a VeraCrypt Volume
`veracrypt --text --mount vctest.vc /mnt --password MySuperSecurePassword1! --pim 0 --keyfiles "" --protect-hidden no --slot 1 --verbose`

# Listing Mounted Volumes
`veracrypt --text --list`

# Dismounting VeraCrypt Volumes
## Method 1: Slot Number
`veracrypt --text --dismount --slot 1`

## Method 2: Mount Directory
`veracrypt --text --dismount /mnt`

## Method 3: Volume File Name
`veracrypt --text --dismount vctest.vc`

or

`veracrypt --text --dismount /home/arcanecode/Documents/vctest.vc`

# Recuperate the hash
`dd if=container.vc of=new.vc bs=1 count=512`