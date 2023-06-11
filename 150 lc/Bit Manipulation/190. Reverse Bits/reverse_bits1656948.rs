// https://leetcode.com/problems/reverse-bits/solutions/1656948/c-2-solutions-easier-vs-faster/
 uint32_t reverseBits(uint32_t n)
 {
 	uint32_t result = 0;
 	string bi_string;

 	// Build the binary string ( in reverse order, no need
 	// to reverse for the result).
 	while (n > 0)
 	{
 		bi_string.append(n % 2 == 1 ? "1": "0");
 		n /= 2;
 	}

 	// if the string is short than 32, add '0' at the end.
 	bi_string.append(32 - bi_string.length(), '0');

 	// Convert binary string to uint value
 	uint32_t seed = 0;
 	for (int i = bi_string.length() - 1; i >= 0; i--)
 	{
 		seed = (0 == seed) ? 1 : seed * 2;
 		result += (bi_string[i] == '1' ? seed: 0);
 	}

 	return result;