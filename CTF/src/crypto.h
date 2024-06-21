#ifndef CRYPTO_H
#define CRYPTO_H

char *encrypt1(char *str, int key);
char *decrypt1(char *str, int key);
char *encrypt2(char *str, char *key);
char *decrypt2(char *str, char *key);
#endif // CRYPTO_H