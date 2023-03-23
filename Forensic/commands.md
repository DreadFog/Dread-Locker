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

# Crack the hash
`hashcat -m 13721 -a 0 -o cracked.txt my_stuff.vc rockyou.txt`

# Result
my_stuff.vc:erica

# Hint
Tu utilises un mdp qui est dans rockyou!
Dont't worry, it is a sha512 encryption ;)