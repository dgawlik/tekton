
#include <vector>
#include <cstdlib>
#include <immintrin.h>

typedef __uint128_t uint128;
typedef u_int8_t byte;


inline uint128 toUint128(byte* bytes){
    return *((uint128*)&bytes[0]);
}

inline byte* toBytes(uint128 num){
    byte* result = new byte[16];
    uint128* view = (uint128*) &result[0];
    *view = num;

    return result;
}


class Tekton {
    public:

    Tekton(byte* key){
        uint128 base = toUint128(key);
       
        keys.push_back(base);
        int shift = 1;
        for(int i=0;i<7;i++){
            keys.push_back(base << shift);
            shift *= 2;
        }

        std::srand(base);

        for(int i=0;i<16;i++){
            invP[P[i]] = i;
        }

        for(int i=0;i<256;i++){
            S[i] = i;
        }

        for(int i=0;i<65000;i++){
            int ia = rand() % 256;
            int ib = rand() % 256;

            int t = S[ia];
            S[ia] = S[ib];
            S[ib] = t;
        }

        mask1 = makeMask(0b01010101010101010101010101010101U);
        mask2 = makeMask(0b00110011001100110011001100110011U);
        mask3 = makeMask(0b00001111000011110000111100001111U);
        mask4 = makeMask(0b00000000111111110000000011111111U);
        mask5 = makeMask(0b00000000000000001111111111111111U);
        mask6 = makeMask(0b00000000000000000000000000000000U, 0b11111111111111111111111111111111U);
        mask7 = makeMask(0b0000000000000000000000000000000000000000000000000000000000000000UL, 0b1111111111111111111111111111111111111111111111111111111111111111UL);


        for(int i=0;i<256;i++){
            invS[S[i]] = i;
        }
    }

    byte* encrypt(byte* payload){
        uint128 state = toUint128(payload);

        state ^= keys[0];
        state = permuteSubstitute(state);

        state = diffusion(state);
        
        state ^= keys[1];
        state ^= keys[2];
        state ^= keys[3];

        return toBytes(state);
    }

    byte* decrypt(byte* cipher){
        uint128 state = toUint128(cipher);

        state ^= keys[3];
        state ^= keys[2];
        state ^= keys[1];
        
        state = diffusion(state);

        state = invPermuteSubstitute(state);
        state ^= keys[0];

        return toBytes(state);
    }

    uint128 makeMask(uint m){
        uint128 result;

        result |= m;
        for(int i=0;i<3;i++){
            result <<= 32;
            result |= m;
        }

        return result;
    }

    uint128 makeMask(uint hi, uint lo){
        uint128 result;

        result |= hi;
        result <<= 32;
        result |= lo;
        result <<= 32;
        result |= hi;
        result <<= 32;
        result |= lo;

        return result;
    }

    uint128 makeMask(ulong hi, ulong lo){
        uint128 result;

        result |= hi;
        result <<= 64;
        result |= lo;

        return result;
    }


    uint128 diffusion(uint128& x){
        uint128 p1 = (x & mask1) << 1;
        uint128 p2 = (x & mask2) << 2;
        uint128 p3 = (x & mask3) << 4;
        uint128 p4 = (x & mask4) << 8;
        uint128 p5 = (x & mask5) << 16;
        uint128 p6 = (x & mask6) << 32;
        uint128 p13 = (x & mask7) << 64;

        uint128 p7 = (x & ~mask1) >> 1;
        uint128 p8 = (x & ~mask2) >> 2;
        uint128 p9 = (x & ~mask3) >> 4;
        uint128 p10 = (x & ~mask4) >> 8;
        uint128 p11 = (x & ~mask5) >> 16;
        uint128 p12 = (x & ~mask6) >> 32;
        uint128 p14 = (x & ~mask7) >> 64;
       
        return x ^ p1 ^ p2 ^ p3 ^ p4 ^ p5 ^ p6 ^ p7 ^ p8 ^p9 ^ p10 ^ p11 ^ p12;
    }

    uint128 permuteSubstitute(uint128& x){
        byte* src = (byte*) &x;
        uint128 dest;
        byte* destView = (byte*) &dest;

        for(int i=0;i<16;i++){
            destView[P[i]] = S[src[i]];
        }

        return dest;
    }

    uint128 invPermuteSubstitute(uint128& x){
        byte* src = (byte*) &x;
        uint128 dest;
        byte* destView = (byte*) &dest;

        for(int i=0;i<16;i++){
            destView[invP[i]] = invS[src[i]];
        }

        return dest;
    }

    private:
    std::vector<uint128> keys;
    int P[16] =    {9, 11, 13, 5,  3,  1,  12,  15,  2,   4,   14,  10,   8,   7,   0,  6};
    int invP[16];
    int S[256];
    int invS[256];
    uint128 mask1, mask2, mask3, mask4, mask5, mask6, mask7;

};