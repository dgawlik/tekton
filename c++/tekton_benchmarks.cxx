
#include <iostream>
#include "tekton.hxx"
#include <chrono>
#include <cstdlib>
#include <AES.h>
#include <immintrin.h>


void measureAESOnChipEncryption(){
  long elapsed = 0;
  unsigned char key[] = { 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f };
  for(int i=0;i<1000000;i++){
    uint128 r = rand();

    byte* payload = toBytes(r);
    std::chrono::system_clock::time_point start = std::chrono::system_clock::now();
    __m128i vKey = *((__m128i*) &key);
    __m128i vPayload = *((__m128i*) payload);
    __m128i vResult =_mm_aesenc_si128(vPayload, vKey);
    vResult =_mm_aesenc_si128(vResult, vKey);
    vResult =_mm_aesenc_si128(vResult, vKey);
    vResult =_mm_aesenc_si128(vResult, vKey);
    vResult =_mm_aesenc_si128(vResult, vKey);
    std::chrono::system_clock::time_point end = std::chrono::system_clock::now();
    elapsed += std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count();
  }
   
   std::cout<< "AES on chip encryption: " << elapsed/1000000 << " ms" << std::endl;
}

void measureAESOnChipDecryption(){
  long elapsed = 0;
  unsigned char key[] = { 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f };
  for(int i=0;i<1000000;i++){
    uint128 r = rand();

    byte* payload = toBytes(r);
    std::chrono::system_clock::time_point start = std::chrono::system_clock::now();
    __m128i vKey = *((__m128i*) &key);
    __m128i vPayload = *((__m128i*) payload);
    __m128i vResult =_mm_aesdec_si128(vPayload, vKey);
    vResult =_mm_aesdec_si128(vResult, vKey);
    vResult =_mm_aesdec_si128(vResult, vKey);
    vResult =_mm_aesdec_si128(vResult, vKey);
    vResult =_mm_aesdec_si128(vResult, vKey);
    std::chrono::system_clock::time_point end = std::chrono::system_clock::now();
    elapsed += std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count();
  }
   
   std::cout<< "AES on chip decryption: " << elapsed/1000000 << " ms" << std::endl;
}


void measureTektonEncryption(){
    
  uint128 key = rand();
  Tekton tek(toBytes(key));


  long elapsed = 0;
  for(int i=0;i<1000000;i++){
    uint128 r = rand();

    byte* payload = toBytes(r);
    std::chrono::system_clock::time_point start = std::chrono::system_clock::now();
    byte* cipher = tek.encrypt(payload);
    std::chrono::system_clock::time_point end = std::chrono::system_clock::now();
    elapsed += std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count();
  }
   
   std::cout<< "Tekton encryption: " << elapsed/1000000 << " ms" << std::endl;

}

void measureAESEncryption(){
    unsigned char key[] = { 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f };
    unsigned int plainLen = 16 * sizeof(unsigned char);

    AES aes(AESKeyLength::AES_128);



  long elapsed = 0;
  for(int i=0;i<1000000;i++){
    uint128 r = rand();

    byte* payload = toBytes(r);
    std::chrono::system_clock::time_point start = std::chrono::system_clock::now();
    aes.EncryptECB(payload, plainLen, key);
    std::chrono::system_clock::time_point end = std::chrono::system_clock::now();
    elapsed += std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count();
  }
   
   std::cout<< "Some AES encryption: " << elapsed/1000000 << " ms" << std::endl;

}



void measureTektonDecryption(){
    
  uint128 key = rand();
  Tekton tek(toBytes(key));


  long elapsed = 0;
  for(int i=0;i<1000000;i++){
    uint128 r = rand();

    byte* payload = toBytes(r);
    byte* cipher = tek.encrypt(payload);
    std::chrono::system_clock::time_point start = std::chrono::system_clock::now();
    byte* result = tek.decrypt(cipher);
    std::chrono::system_clock::time_point end = std::chrono::system_clock::now();
    elapsed += std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count();
  }
   
   std::cout<< "Tekton decryption: " << elapsed/1000000 << " ms" << std::endl;

}


void measureAESDecryption(){
    unsigned char key[] = { 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f };
    unsigned int plainLen = 16 * sizeof(unsigned char);

    AES aes(AESKeyLength::AES_128);



  long elapsed = 0;
  for(int i=0;i<1000000;i++){
    uint128 r = rand();

    byte* payload = toBytes(r);
    byte* c = aes.EncryptECB(payload, plainLen, key);
    std::chrono::system_clock::time_point start = std::chrono::system_clock::now();
    aes.DecryptECB(c, plainLen, key);
    std::chrono::system_clock::time_point end = std::chrono::system_clock::now();
    elapsed += std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count();
  }
   
   std::cout<< "Some AES decryption: " << elapsed/1000000 << " ms" << std::endl;

}



int main(){
    measureTektonEncryption();
    measureTektonDecryption();
    measureAESEncryption();
    measureAESDecryption();
    measureAESOnChipEncryption();
    measureAESOnChipDecryption();
    return 0;
}