import timeit, simu83
from rom_reader import reader_gb

rom_path = "/home/hyli360/my_project/SiMu83/src/test.bin"
rom = reader_gb(rom_path)

soc = simu83.SoC(rom) # type: ignore

def main():
    while not soc.halt():
        soc.one_step()
        soc.disp()

if __name__ == "__main__":
    main()

main()