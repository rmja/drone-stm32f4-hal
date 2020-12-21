SECTIONS
{
    # Add a SDRAM heap section in the SDRAM memory region
    # It is important that NOLOAD is specified, to make sure that
    # the region is not zero'd during reset, before the fmc driver has
    # setup the FMC peripheral and the memory module.
    .sdram_heap (NOLOAD) :
    {
        . = ALIGN(4);
        *(.sdram_heap);
    } > SDRAM
}