import re
import sys

def main(params):
    file = open(params[0])
    for line in file:
        if "rw!" in line and not "//" in line and not "trs" in line:
            m = re.findall('\".+?\"',line)
            if m:
                start = m[1].replace("?","")
                end = m[2].replace("?","")
                print('( ' + start + ',' + end +' ),')




if __name__ == '__main__':
    main(sys.argv[1:])