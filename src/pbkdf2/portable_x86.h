#ifndef KECCACK_PORTABLE_X86_H
#define KECCACK_PORTABLE_X86_H

#if defined(CPU_X86) && (defined(COMPILER_MSVC) || defined(COMPILER_GCC))
	#define X86ASM

	/* gcc 2.95 royally screws up stack alignments on variables */
	#if ((defined(COMPILER_MSVC) && (COMPILER_MSVC >= COMPILER_MSVC_VS6PP)) || (defined(COMPILER_GCC) && (COMPILER_GCC >= 30000)))
		#define X86ASM_SSE
		#define X86ASM_SSE2
	#endif
	#if ((defined(COMPILER_MSVC) && (COMPILER_MSVC >= COMPILER_MSVC_VS2005)) || (defined(COMPILER_GCC) && (COMPILER_GCC >= 40102)))
		#define X86ASM_SSSE3
	#endif
	#if ((defined(COMPILER_MSVC) && (COMPILER_MSVC >= COMPILER_MSVC_VS2010SP1)) || (defined(COMPILER_GCC) && (COMPILER_GCC >= 40400)))
		#define X86ASM_AVX
		#define X86ASM_XOP
	#endif
	#if ((defined(COMPILER_MSVC) && (COMPILER_MSVC >= COMPILER_MSVC_VS2012)) || (defined(COMPILER_GCC) && (COMPILER_GCC >= 40700)))
		#define X86ASM_AVX2
	#endif
#endif

#if defined(CPU_X86_64) && defined(COMPILER_GCC)
	#define X86_64ASM
	#define X86_64ASM_SSE2
	#if (COMPILER_GCC >= 40102)
		#define X86_64ASM_SSSE3
	#endif
	#if (COMPILER_GCC >= 40400)
		#define X86_64ASM_AVX
		#define X86_64ASM_XOP
	#endif
	#if (COMPILER_GCC >= 40700)
		#define X86_64ASM_AVX2
	#endif
#endif

#if defined(COMPILER_MSVC) && (defined(CPU_X86_FORCE_INTRINSICS) || defined(CPU_X86_64))
	#define X86_INTRINSIC
	#if defined(CPU_X86_64) || defined(X86ASM_SSE)
		#define X86_INTRINSIC_SSE
	#endif
	#if defined(CPU_X86_64) || defined(X86ASM_SSE2)
		#define X86_INTRINSIC_SSE2
	#endif
	#if (COMPILER_MSVC >= COMPILER_MSVC_VS2005)
		#define X86_INTRINSIC_SSSE3
	#endif
	#if (COMPILER_MSVC >= COMPILER_MSVC_VS2010SP1)
		#define X86_INTRINSIC_AVX
		#define X86_INTRINSIC_XOP
	#endif
	#if (COMPILER_MSVC >= COMPILER_MSVC_VS2012)
		#define X86_INTRINSIC_AVX2
	#endif
#endif

#if defined(COMPILER_GCC) && defined(CPU_X86_FORCE_INTRINSICS)
	#define X86_INTRINSIC
	#if defined(__SSE__)
		#define X86_INTRINSIC_SSE
	#endif
	#if defined(__SSE2__)
		#define X86_INTRINSIC_SSE2
	#endif
	#if defined(__SSSE3__)
		#define X86_INTRINSIC_SSSE3
	#endif
	#if defined(__AVX__)
		#define X86_INTRINSIC_AVX
	#endif
	#if defined(__XOP__)
		#define X86_INTRINSIC_XOP
	#endif
	#if defined(__AVX2__)
		#define X86_INTRINSIC_AVX2
	#endif
#endif

/* only use simd on windows (or SSE2 on gcc)! */
#if defined(CPU_X86_FORCE_INTRINSICS) || defined(X86_INTRINSIC)
	#if defined(X86_INTRINSIC_SSE)
		#include <mmintrin.h>
		#include <xmmintrin.h>
		typedef __m64 qmm;
		typedef __m128 xmm;
		typedef __m128d xmmd;
	#endif
	#if defined(X86_INTRINSIC_SSE2)
		#include <emmintrin.h>
		typedef __m128i xmmi;
	#endif
	#if defined(X86_INTRINSIC_SSSE3)
		#include <tmmintrin.h>
	#endif
	#if defined(X86_INTRINSIC_AVX)
		#include <immintrin.h>
	#endif
	#if defined(X86_INTRINSIC_XOP)
		#if defined(COMPILER_MSVC)
			#include <intrin.h>
		#else
			#include <x86intrin.h>
		#endif
	#endif
	#if defined(X86_INTRINSIC_AVX2)
		typedef __m256i ymmi;
	#endif
#endif

#if defined(X86_INTRINSIC_SSE2)
	typedef union packedelem8_t {
		uint8_t u[16];
		xmmi v;	
	} packedelem8;

	typedef union packedelem32_t {
		uint32_t u[4];
		xmmi v;	
	} packedelem32;

	typedef union packedelem64_t {
		uint64_t u[2];
		xmmi v;	
	} packedelem64;
#else
	typedef union packedelem8_t {
		uint8_t u[16];
		uint32_t dw[4];		
	} packedelem8;

	typedef union packedelem32_t {
		uint32_t u[4];
		uint8_t b[16];
	} packedelem32;

	typedef union packedelem64_t {
		uint64_t u[2];
		uint8_t b[16];
	} packedelem64;
#endif

#if defined(X86_INTRINSIC_SSSE3)
	static const packedelem8 ALIGN(16) ssse3_rotl16_32bit      = {{2,3,0,1,6,7,4,5,10,11,8,9,14,15,12,13}};
	static const packedelem8 ALIGN(16) ssse3_rotl8_32bit       = {{3,0,1,2,7,4,5,6,11,8,9,10,15,12,13,14}};
#endif

/*
	x86 inline asm for gcc/msvc. usage:

	asm_naked_fn_proto(return_type, name) (type parm1, type parm2..)
	asm_naked_fn(name)
		a1(..)
		a2(.., ..)
		a3(.., .., ..)
		64bit OR 0 paramters: a1(ret)
		32bit AND n parameters: aret(4n), eg aret(16) for 4 parameters
	asm_naked_fn_end(name)
*/

#if defined(X86ASM) || defined(X86_64ASM)

#if defined(COMPILER_MSVC)
	#pragma warning(disable : 4731) /* frame pointer modified by inline assembly */
	#define a1(x) __asm {x}
	#define a2(x, y) __asm {x, y}
	#define a3(x, y, z) __asm {x, y, z}
	#define a4(x, y, z, w) __asm {x, y, z, w}
	#define aj(x) __asm {x}
	#define asm_align8 a1(ALIGN 8)
	#define asm_align16 a1(ALIGN 16)

	#define asm_calling_convention STDCALL
	#define aret(n) a1(ret n)
	#define asm_naked_fn_proto(type, fn) static NAKED type asm_calling_convention fn
	#define asm_naked_fn(fn) {
	#define asm_naked_fn_end(fn) }
#elif defined(COMPILER_GCC)
	#define GNU_AS1(x) #x ";\n"
	#define GNU_AS2(x, y) #x ", " #y ";\n"
	#define GNU_AS3(x, y, z) #x ", " #y ", " #z ";\n"
	#define GNU_AS4(x, y, z, w) #x ", " #y ", " #z ", " #w ";\n"
	#define GNU_ASFN(x) "\n_" #x ":\n" #x ":\n"
	#define GNU_ASJ(x) ".att_syntax prefix\n" #x "\n.intel_syntax noprefix\n"

	#define a1(x) GNU_AS1(x)
	#define a2(x, y) GNU_AS2(x, y)
	#define a3(x, y, z) GNU_AS3(x, y, z)
	#define a4(x, y, z, w) GNU_AS4(x, y, z, w)
	#define aj(x) GNU_ASJ(x)
	#define asm_align8 ".p2align 3,,7"
	#define asm_align16 ".p2align 4,,15"

	#if defined(OS_WINDOWS)
		#define asm_calling_convention CDECL
		#define aret(n) a1(ret)

		#if defined(X86_64ASM)
			#define asm_naked_fn(fn) ; __asm__ ( \
				".text\n"                        \
				asm_align16 GNU_ASFN(fn)         \
				"subq $136, %rsp;"               \
			 	"movdqa %xmm6, 0(%rsp);"         \
				"movdqa %xmm7, 16(%rsp);"        \
			 	"movdqa %xmm8, 32(%rsp);"        \
				"movdqa %xmm9, 48(%rsp);"        \
			 	"movdqa %xmm10, 64(%rsp);"       \
				"movdqa %xmm11, 80(%rsp);"       \
				"movdqa %xmm12, 96(%rsp);"       \
				"movq %rdi, 112(%rsp);"          \
				"movq %rsi, 120(%rsp);"          \
				"movq %rcx, %rdi;"               \
				"movq %rdx, %rsi;"               \
				"movq %r8, %rdx;"                \
				"movq %r9, %rcx;"                \
				"call 1f;"                       \
				"movdqa 0(%rsp), %xmm6;"         \
				"movdqa 16(%rsp), %xmm7;"        \
				"movdqa 32(%rsp), %xmm8;"        \
				"movdqa 48(%rsp), %xmm9;"        \
				"movdqa 64(%rsp), %xmm10;"       \
				"movdqa 80(%rsp), %xmm11;"       \
				"movdqa 96(%rsp), %xmm12;"       \
				"movq 112(%rsp), %rdi;"          \
				"movq 120(%rsp), %rsi;"          \
				"addq $136, %rsp;"               \
				"ret;"                           \
				".intel_syntax noprefix;"        \
				".p2align 4,,15;"                \
				"1:;"
		#else
			#define asm_naked_fn(fn) ; __asm__ (".intel_syntax noprefix;\n.text\n" asm_align16 GNU_ASFN(fn)
		#endif
	#else
		#define asm_calling_convention STDCALL
		#define aret(n) a1(ret n)		
		#define asm_naked_fn(fn) ; __asm__ (".intel_syntax noprefix;\n.text\n" asm_align16 GNU_ASFN(fn)
	#endif

	#define asm_naked_fn_proto(type, fn) extern type asm_calling_convention fn
	#define asm_naked_fn_end(fn) ".att_syntax prefix;\n" );

	#define asm_gcc() __asm__ __volatile__(".intel_syntax noprefix;\n"
	#define asm_gcc_parms() ".att_syntax prefix;"
	#define asm_gcc_trashed() __asm__ __volatile__("" :::
	#define asm_gcc_end() );
#else
	need x86 asm
#endif

#endif /* X86ASM || X86_64ASM */


#if defined(CPU_X86) || defined(CPU_X86_64)

typedef enum cpu_flags_x86_t {
	cpu_mmx = 1 << 0,
	cpu_sse = 1 << 1,
	cpu_sse2 = 1 << 2,
	cpu_sse3 = 1 << 3,
	cpu_ssse3 = 1 << 4,
	cpu_sse4_1 = 1 << 5,
	cpu_sse4_2 = 1 << 6,
	cpu_avx = 1 << 7,
	cpu_xop = 1 << 8,
	cpu_avx2 = 1 << 9
} cpu_flags_x86;

typedef enum cpu_vendors_x86_t {
	cpu_nobody,
	cpu_intel,
	cpu_amd
} cpu_vendors_x86;

typedef struct x86_regs_t {
	uint32_t eax, ebx, ecx, edx;
} x86_regs;

#if defined(X86ASM)
asm_naked_fn_proto(int, has_cpuid)(void)
asm_naked_fn(has_cpuid)
	a1(pushfd)
	a1(pop eax)
	a2(mov ecx, eax)
	a2(xor eax, 0x200000)
	a1(push eax)
	a1(popfd)
	a1(pushfd)
	a1(pop eax)
	a2(xor eax, ecx)
	a2(shr eax, 21)
	a2(and eax, 1)
	a1(push ecx)
	a1(popfd)
	a1(ret)
asm_naked_fn_end(has_cpuid)
#endif /* X86ASM */

/* enable the highest system-wide option */
#if defined(SCRYPT_CHOOSE_COMPILETIME)
	#if !defined(__AVX2__)
		#undef X86_64ASM_AVX2
		#undef X86ASM_AVX2
		#undef X86_INTRINSIC_AVX2
	#endif
	#if !defined(__XOP__)
		#undef X86_64ASM_XOP
		#undef X86ASM_XOP
		#undef X86_INTRINSIC_XOP
	#endif
	#if !defined(__AVX__)
		#undef X86_64ASM_AVX
		#undef X86ASM_AVX
		#undef X86_INTRINSIC_AVX
	#endif
	#if !defined(__SSSE3__)
		#undef X86_64ASM_SSSE3
		#undef X86ASM_SSSE3
		#undef X86_INTRINSIC_SSSE3
	#endif
	#if !defined(__SSE2__)
		#undef X86_64ASM_SSE2
		#undef X86ASM_SSE2
		#undef X86_INTRINSIC_SSE2
	#endif
#endif

#endif /* defined(CPU_X86) || defined(CPU_X86_64) */

#endif // KECCACK_PORTABLE_X86_H
