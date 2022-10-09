#include <gtest/gtest.h>
#include <cstdlib>
#include "tekton.hxx"

TEST(Tekton, TestConversion) {

  uint128 num = 11231231414414;

  ASSERT_EQ(11231231414414, toUint128(toBytes(num)));

}

TEST(Tekton, TestDiffusion){
  uint128 num = 1;

  Tekton tek(toBytes(11231231414414));
  
  auto step = tek.diffusion(num);

  ASSERT_EQ(num, tek.diffusion(step));
}


TEST(Tekton, TestEncryptDecrypt) {

  uint128 key = rand();
  Tekton tek(toBytes(key));

  for(int i=0;i<10000;i++){
    uint128 r = rand();

    byte* payload = toBytes(r);
    byte* cipher = tek.encrypt(payload);
    byte* result = tek.decrypt(cipher);

    uint128 r2 = toUint128(result);

    ASSERT_TRUE(r == r2);
  }

}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
