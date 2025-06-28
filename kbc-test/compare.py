import sys

def main():
    with open('/home/michi/Documents/thesis/KBC/kbc-test/kbc_math.txt', 'r') as kbc:
        kbc_lines = [line.strip() for line in kbc]
    with open('/home/michi/Documents/thesis/KBC/kbc-test/math.txt', 'r') as egg:
        egg_lines = [line.strip() for line in egg]
    i = 0
    runs = []
    with open('/home/michi/Documents/thesis/KBC/kbc-test/kbc-egg_comp.txt', 'w') as f:
        kbcSum = 0
        eggSum = 0
        while i < len(kbc_lines):
            if kbc_lines[i] == "":
                i+=1
                continue
            kbcAvg = float(kbc_lines[i].split()[2])
            eggAvg = float(egg_lines[i].split()[2])
            speedup = eggAvg/kbcAvg
            i+=1
            kbcStopReason = kbc_lines[i].split()[2]
            eggStopReason = egg_lines[i].split()[2]
            i+=1
            kbcIters = kbc_lines[i].split()[1]
            eggIters = egg_lines[i].split()[1]
            i+=1
            original = kbc_lines[i].split(':')[1]
            i+=1
            kbcSimplified = kbc_lines[i].split(':')[1]
            eggSimplified = egg_lines[i].split(':')[1]
            i+=1
            runs.append((kbcAvg, eggAvg, speedup, kbcStopReason, eggStopReason, kbcIters, eggIters, original, kbcSimplified, eggSimplified))
            kbcSum += kbcAvg
            eggSum += eggAvg

        runs.sort(key=lambda x: x[2])
        for run in runs:
            f.write("\n\nOriginal term:" + run[7] + "\nSimplified:\n\tKBC:" + run[8] + "\n\tEgg:" + run[9] + "\nIters:\n\tKBC:" + run[5] + "\n\tEgg:" + run[6] +"\nStop reason:\n\tKBC:" + run[3]
                    + "\n\tEgg:" + run[4] + "\nRan for:\n\tKBC:" + str(run[0]) + "s\n\tEgg:" + str(run[1]) + "s\nSpeedUp:" + str(run[2]))
        f.write("\n\nSpeedUp over all terms:" + str(eggSum/kbcSum) + "\n\tKBCSum:" + str(kbcSum) + "\n\tEggSum:" + str(eggSum))

if __name__ == '__main__':
    main()
    