import numpy as np
import math


def dft(x):
    """
    Plain and basic fourier transform with a runtime of O(n^2)
    """
    N = len(x)

    # compute dft on input array
    DFT = [0] * N
    for k in range(N):
        t = 0
        # inner loop over range(N) == O(n^2)
        for n in range(N):
            t += x[n] * math.e ** (-2j * math.pi * k * n / N)
        DFT[k] = t

    return DFT


def fft(x):
    """
    Implementation of a basic fourier transformation algorithm that runs
    in O(nlogn) time. The input list has to be a power of 2
    """
    N = len(x)

    # base case
    if N == 1:
        return x

    # divide into even and odd parts
    # and run fft on them
    even = fft(x[0::2])
    odd = fft(x[1::2])

    # compute the dft
    DFT = [0] * N
    for k in range(N // 2):
        t = odd[k] * math.e ** (-2j * math.pi * k / N)
        DFT[k] = even[k] + t
        DFT[k + N // 2] = even[k] - t

    return DFT


def fft_cooley_turkey(x):
    """
    Implementation of the Cooley-Tukey fft algorithm. Uses complex numbers to
    determine how to combine the left and right side of the lists. This function
    also assumes list length is a power of 2
    """
    n = len(x)
    if n == 1:
        # Base case: if the length of x is 1, return x
        return x
    else:
        # Divide the input list x into even and odd sublists
        even = fft_cooley_turkey(x[0::2])
        odd = fft_cooley_turkey(x[1::2])
        # Compute the complex exponential factors for combining the sublists
        factor = [math.e ** (-2j * math.pi * k / n) * odd[k] for k in range(n // 2)]
        # Combine the even and odd sublists using the complex exponential factors
        return [even[k] + factor[k] for k in range(n // 2)] + [
            even[k] - factor[k] for k in range(n // 2)
        ]


def fft_pad(x, func):
    """
    Pads the input list to a power of 2, and uses the passed in
    fft alogorithm. Depending on the list length, padding can be quite inaccurate
    """
    N = len(x)
    padded_N = 2 ** math.ceil(math.log2(N))
    x_padded = x + [0] * (padded_N - N)

    return func(x_padded)


if __name__ == "__main__":
    lst = [0.5, 0.7, 0.3, 0.56, 0.56, 0.34, 0.77, 0.34]
    print("NUMPY:")
    print(np.fft.fft(lst))
    print("DFT:")
    print(np.array(fft_pad(lst, dft)))
    print("FFT:")
    print(np.array(fft_pad(lst, fft)))
    print("COOLEY-TURKEY:")
    print(np.array(fft_pad(lst, fft_cooley_turkey)))
