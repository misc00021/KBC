import sys

def main():
    with open('/home/michi/Documents/thesis/KBC/CraneliftOutOrder', 'r') as file:
        lines = [line.strip() for line in file]
        axioms = []
        rules = []
        for line in lines:
            spl = line.split(' ')
            if len(spl) >= 3:
                if spl[0] == "Axiom":
                    axioms.append(line)
                elif (list(spl[0])[0]) >= '0' and (list(spl[0])[0]) <= '9':
                    rules.append(line)
    with open('/home/michi/Documents/thesis/KBC/CraneLiftDiff.txt', 'w') as f:
        for i in range(len(axioms)):
            lhsAx = axioms[i].split(':')[1].split('=')[0]
            lhsRu = rules[i].split('.')[1].split('-')[0]
            if lhsAx != lhsRu:
                f.write(axioms[i] + "\nbecomes\n" + rules[i] + "\n")

if __name__ == '__main__':
    main()
    