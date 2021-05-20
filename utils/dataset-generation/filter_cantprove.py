import sys

def main(params):
    f = open(params[0])
    filtered = open('./results_cantprove.csv','w')
    for l in f.readlines():
        r = l.split(',')[2]
        if not r == '1':
            filtered.write(l)

if __name__=='__main__':
    main(sys.argv[1:])