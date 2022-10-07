
#include <vector>
#include <cstdlib>
#include <immintrin.h>

typedef __uint128_t uint128;
typedef u_int8_t byte;


inline uint128 toUint128(byte* bytes){
    return *((uint128*)bytes);
}

inline uint128 intToUint128(unsigned int ints[4]){
    return *((uint128*)ints);
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

        unsigned int _mask1a[4] = {0b01010101010101010101010101010101,0b01010101010101010101010101010101,0b01010101010101010101010101010101,0b01010101010101010101010101010101};
        unsigned int _mask1b[4] = {0b10101010101010101010101010101010,0b10101010101010101010101010101010,0b10101010101010101010101010101010,0b10101010101010101010101010101010};
        unsigned int _mask2a[4] = {0b00110011001100110011001100110011, 0b00110011001100110011001100110011,0b00110011001100110011001100110011,0b00110011001100110011001100110011};
        unsigned int _mask2b[4] = {0b11001100110011001100110011001100,0b11001100110011001100110011001100,0b11001100110011001100110011001100,0b11001100110011001100110011001100};
        unsigned int _mask3a[4] = {0b00001111000011110000111100001111,0b00001111000011110000111100001111,0b00001111000011110000111100001111,0b00001111000011110000111100001111};
        unsigned int _mask3b[4] = {0b11110000111100001111000011110000, 0b11110000111100001111000011110000,0b11110000111100001111000011110000,0b11110000111100001111000011110000};
        unsigned int _mask4a[4] = {0b00000000111111110000000011111111, 0b00000000111111110000000011111111,0b00000000111111110000000011111111,0b00000000111111110000000011111111};
        unsigned int _mask4b[4] = {0b11111111000000001111111100000000,0b11111111000000001111111100000000,0b11111111000000001111111100000000,0b11111111000000001111111100000000};
        unsigned int _mask5a[4] = {0b00000000000000001111111111111111,0b00000000000000001111111111111111,0b00000000000000001111111111111111,0b00000000000000001111111111111111};
        unsigned int _mask5b[4] = {0b11111111111111110000000000000000,0b11111111111111110000000000000000,0b11111111111111110000000000000000,0b11111111111111110000000000000000};
        unsigned int _mask6b[4] = {0b00000000000000000000000000000000, 0b11111111111111111111111111111111,0b00000000000000000000000000000000, 0b11111111111111111111111111111111};
        unsigned int _mask6a[4] = {0b11111111111111111111111111111111, 0b00000000000000000000000000000000, 0b11111111111111111111111111111111, 0b00000000000000000000000000000000};

        mask1a = intToUint128(_mask1a);
        mask1b = intToUint128(_mask1b);
        mask2a = intToUint128(_mask2a);
        mask2b = intToUint128(_mask2b);
        mask3a = intToUint128(_mask3a);
        mask3b = intToUint128(_mask3b);
        mask4a = intToUint128(_mask4a);
        mask4b = intToUint128(_mask4b);
        mask5a = intToUint128(_mask5a);
        mask5b = intToUint128(_mask5b);
        mask6a = intToUint128(_mask6a);
        mask6b = intToUint128(_mask6b);
        


        for(int i=0;i<256;i++){
            invS[S[i]] = i;
        }
    }

    byte* encrypt(byte* payload){
        uint128 state = toUint128(payload);

        state ^= keys[0];

        state = diffusionOptimized(state);
        state = permuteSubstitute(state);
        state ^= keys[1];

        state = diffusionOptimized(state);
        state = permuteSubstitute(state);
        state ^= keys[2];

        state = diffusionOptimized(state);
        state = permuteSubstitute(state);
        state ^= keys[3];

        return toBytes(state);
    }

    byte* decrypt(byte* cipher){
        uint128 state = toUint128(cipher);

        state ^= keys[3];
        state = invPermuteSubstitute(state);
        state = diffusionOptimized(state);

        state ^= keys[2];
        state = invPermuteSubstitute(state);
        state = diffusionOptimized(state);

        state ^= keys[1];
        state = invPermuteSubstitute(state);
        state = diffusionOptimized(state);

        state ^= keys[0];

        return toBytes(state);
    }


    uint128 diffusion(uint128& x){
        uint128 p1 = (x & mask1a) << 1;
        uint128 p2 = (x & mask2a) << 2;
        uint128 p3 = (x & mask3a) << 4;
        uint128 p4 = (x & mask4a) << 8;
        uint128 p5 = (x & mask5a) << 16;
        uint128 p6 = (x & mask6a) << 32;

        uint128 p7 = (x & mask1b) >> 1;
        uint128 p8 = (x & mask2b) >> 2;
        uint128 p9 = (x & mask3b) >> 4;
        uint128 p10 = (x & mask4b) >> 8;
        uint128 p11 = (x & mask5b) >> 16;
        uint128 p12 = (x & mask6b) >> 32;
       
        return x ^ p1 ^ p2 ^ p3 ^ p4 ^ p5 ^ p6 ^ p7 ^ p8 ^p9 ^ p10 ^ p11 ^ p12;
    }


    uint128 diffusionOptimized(uint128& x){
        __m128i vx = _mm_loadu_si128((const __m128i_u *) &x);


        __m128i vtmp;
        __m128i vmask;

        vmask = _mm_loadu_si128((const __m128i_u *) &mask1a);
        vtmp = _mm_and_si128(vmask, vx);
        __m128i vp1 = _mm_slli_epi64(vtmp, 1);

        vmask = _mm_loadu_si128((const __m128i_u *) &mask2a);
        vtmp = _mm_and_si128(vmask, vx);
        __m128i vp2 = _mm_slli_epi64(vtmp, 2);

        vmask = _mm_loadu_si128((const __m128i_u *) &mask3a);
        vtmp = _mm_and_si128(vmask, vx);
        __m128i vp3 = _mm_slli_epi64(vtmp, 4);

        vmask = _mm_loadu_si128((const __m128i_u *) &mask4a);
        vtmp = _mm_and_si128(vmask, vx);
        __m128i vp4 = _mm_bslli_si128(vtmp, 1);

        vmask = _mm_loadu_si128((const __m128i_u *) &mask5a);
        vtmp = _mm_and_si128(vmask, vx);
        __m128i vp5 = _mm_bslli_si128(vtmp, 2);

        vmask = _mm_loadu_si128((const __m128i_u *) &mask6a);
        vtmp = _mm_and_si128(vmask, vx);
        __m128i vp6 = _mm_bslli_si128(vtmp, 4);


         vmask = _mm_loadu_si128((const __m128i_u *) &mask1b);
        vtmp = _mm_and_si128(vmask, vx);
        __m128i vp7 = _mm_srli_epi64(vtmp, 1);

         vmask = _mm_loadu_si128((const __m128i_u *) &mask2b);
        vtmp = _mm_and_si128(vmask, vx);
        __m128i vp8 = _mm_srli_epi64(vtmp, 2);

        vmask = _mm_loadu_si128((const __m128i_u *) &mask3b);
        vtmp = _mm_and_si128(vmask, vx);
        __m128i vp9 = _mm_srli_epi64(vtmp, 4);

        vmask = _mm_loadu_si128((const __m128i_u *) &mask4b);
        vtmp = _mm_and_si128(vmask, vx);
        __m128i vp10 = _mm_bsrli_si128(vtmp, 1);

        vmask = _mm_loadu_si128((const __m128i_u *) &mask5b);
        vtmp = _mm_and_si128(vmask, vx);
        __m128i vp11 = _mm_bsrli_si128(vtmp, 2);

        vmask = _mm_loadu_si128((const __m128i_u *) &mask6b);
        vtmp = _mm_and_si128(vmask, vx);
        __m128i vp12 = _mm_bsrli_si128(vtmp, 4);
        

        __m128i vp13 = _mm_xor_si128(vp1, vp2);
        __m128i vp14 = _mm_xor_si128(vp3, vp4);
        __m128i vp15 = _mm_xor_si128(vp5, vp6);
        __m128i vp16 = _mm_xor_si128(vp7, vp8);
        __m128i vp17 = _mm_xor_si128(vp9, vp10);
        __m128i vp18 = _mm_xor_si128(vp11, vp12);

        __m128i vp19 = _mm_xor_si128(vp13, vp14);
        __m128i vp20 = _mm_xor_si128(vp15, vp16);
        __m128i vp21 = _mm_xor_si128(vp17, vp18);

        __m128i vp22 = _mm_xor_si128(vp19, vp20);
        vx = _mm_xor_si128(vx, vp21);
        vx = _mm_xor_si128(vx, vp22);

        uint128 result;
        _mm_storeu_si128((__m128i_u*)&result, vx);
       
        return result;
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
    uint128 mask1a, mask1b;
    uint128 mask2a, mask2b;
    uint128 mask3a, mask3b;
    uint128 mask4a, mask4b;
    uint128 mask5a, mask5b;
    uint128 mask6a, mask6b;

};