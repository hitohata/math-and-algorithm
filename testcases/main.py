import os

def main():
    for i in range(105):
        path = f'./{str(i).zfill(3)}'
        os.mkdir(path=path)
        os.mkdir(path=f"{path}/in")
        os.mkdir(path=f"{path}/out")


if __name__ == '__main__':
    main()
