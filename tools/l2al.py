with open("tmp.l8", "rb") as f_in:
    with open("tmp.al88", "wb") as f_out:
        byte = f_in.read(1)
        while byte:
            byte = f_in.read(1)
            f_out.write(byte)
            f_out.write(bytes([0xff]))

