#ifndef PBKDF2_H
#define PBKDF2_H

#include <stdint.h>
#include <stdlib.h>

void pbkdf2(const uint8_t *password, size_t password_len, const uint8_t *salt,
            size_t salt_len, uint64_t N, uint8_t *out, size_t bytes);

#endif // PBKDF2_H
