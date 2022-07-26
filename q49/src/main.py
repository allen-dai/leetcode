import json


def main(strs):
    ans = {}
    temp = [0 for _ in range(26)]
    for s in strs:
        hash = temp.copy()
        for c in s:
            i = ord(c) - ord('a')
            hash[i] += 1
        h = str(hash)
        if h in ans:
            (ans[h]).append(s)
        else:
            ans[h] = [s]
    return ans.values()


print(ord('a'))
