
from ctypes import*
# give location of dll
mydll = cdll.LoadLibrary("C:/Users/illemonati/Desktop/stuff_to_compile_on_windows/rust/image_of_the_day_nasa/image_of_the_day_nasa_2/target/release/image_of_the_day_nasa_2.dll")
mydll.run()
