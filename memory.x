MEMORY
{
  /* nRF52840 with Adafruit_nRF52_Bootloader / UF2 bootloader.
     The app starts at 0x1000. Flash above 0xD4000 is reserved for
     RMK storage at 0xD4000..0xF4000 and the bootloader at 0xF4000. */
  FLASH : ORIGIN = 0x00001000, LENGTH = 844K
  RAM : ORIGIN = 0x20000008, LENGTH = 255K
}
