SECTIONS
{
    # Add a SDRAM heap section in the SDRAM memory region
    # It is important that NOLOAD is specified, to make sure that
    # the region is not zero'd during reset, before the fmc driver has
    # setup the FMC peripheral and the memory module.
    .sdram_heap (NOLOAD) :
    {
        . = ALIGN(4);
        . += 6K; # The size of the "slow" heap.
                 # The size must match the size specified in the [heap.slow] section in Drone.toml.
    } > SDRAM
}