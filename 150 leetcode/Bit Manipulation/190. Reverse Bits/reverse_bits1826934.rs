// https://leetcode.com/problems/reverse-bits/solutions/1826934/pure-bitwise-operator-w-o-loop/
uint32_t reverseBits(uint32_t n) {
    // printf("%08x\n", n);
    n = ((n & 0xFFFF0000) >> 16) | ((n & 0x0000FFFF) << 16);
    // printf("%08x\n", n);
    n = ((n & 0xFF00FF00) >> 8) | ((n & 0x00FF00FF) << 8);
    // printf("%08x\n", n);
    n = ((n & 0xF0F0F0F0) >> 4) | ((n & 0x0F0F0F0F) << 4);
    // printf("%08x\n", n);
    n = ((n & 0xCCCCCCCC) >> 2) | ((n & 0x33333333) << 2);
    // printf("%08x\n", n);
    n = ((n & 0xAAAAAAAA) >> 1) | ((n & 0x55555555) << 1);
    // printf("%08x\n", n);
    return n;
}