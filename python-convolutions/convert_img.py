from PIL import Image
import numpy as np


def img_to_3d_arr():
    # Open the webp image using Pillow
    im = Image.open("./mario.webp")

    # Convert the image to RGB mode if it isn't already
    if im.mode != "RGB":
        im = im.convert("RGB")

    arr = np.array(im).tolist()

    print(arr)


def img_to_2d_gray():
    # Open the image and convert to grayscale
    image = Image.open("./mario.webp").convert("L")

    # Get the pixel data as a list of values between 0 and 255
    pixels = list(image.getdata())

    # Normalize the pixel values to be between 0 and 1
    pixels = [x / 255 for x in pixels]

    # Check for transparent pixels and set them to 0
    alpha = image.split()[-1]
    if alpha:
        alpha_pixels = list(alpha.getdata())
        pixels = [pixels[i] if alpha_pixels[i] > 0 else 0 for i in range(len(pixels))]

    # Reshape the pixel list into a 2D array
    width, height = image.size
    pixels_2d = [pixels[i : i + width] for i in range(0, len(pixels), width)]
    print(pixels_2d)


if __name__ == "__main__":
    img_to_2d_gray()
