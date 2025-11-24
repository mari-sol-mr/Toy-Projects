import cv2
import numpy as np
from pyzbar.pyzbar import decode

# Global variables
drawing = False
current_screen = "main"

cap = cv2.VideoCapture(0)
# Create a resizable window
cv2.namedWindow("Barcode Scanner", cv2.WINDOW_NORMAL)

# Resize the window
cv2.resizeWindow("Barcode Scanner", 600, 600)  # width=800, height=600

def capture_barcode() -> str:
    
    
    if not cap.isOpened():
        print("Cannot open camera")
        exit()

    print("Press 'q' to quit")

    while True:
        # Capture frame-by-frame
        ret, frame = cap.read()

        # If frame is read correctly, ret is True
        if not ret:
            print("Can't receive frame. Exiting...")
            break

        cv2.imshow('Barcode Scanner', frame)

        # Decode barcodes
        barcodes = decode(frame)
        # cv2.waitKey(4000)
        # cv2.waitKey(0) 
        # barcodes = ["1234567"]
        if barcodes:
            return barcodes[0].data.decode("utf-8")
            # return "1234567"
        # Press 'q' to exit
        if cv2.waitKey(1) & 0xFF == ord('q'):
            break

    cap.release()


def x_y_in_rectangle(x, y, rectanlge):
    return rectanlge[0] <= x <= rectanlge[0] + rectanlge[2] and \
        rectanlge[1] <= y <= rectanlge[1] + rectanlge[3]

# Mouse callback function
def scan_barcode_button_callback(event, x, y, flags, param):
    global current_screen
    if event == cv2.EVENT_LBUTTONDOWN:
        match current_screen:
            case "main":
                # Check if the click is within the button's boundaries
                if x_y_in_rectangle(x, y, main_button_rect):
                    print("Button clicked!")
                    current_screen = "confirm_barcode"
                    barcode = capture_barcode()
                    confirmation_screen(barcode)
            case "confirm_barcode":
                    # confirmation_screen(barcode)
                    print("done displaying barcode")

            
            

# Create a black image
window_height = 600
window_width = 600
window_background_color = 255 # white
app_background_img = np.full((window_width, window_height, 3), window_background_color, np.uint8)

main_button_width = 200
main_button_height = 50
main_button_rect = (round(window_width*0.5) - round(main_button_width*0.5), round(window_height*0.5) - round(main_button_height*0.5), 200, 50) # (x, y, width, height)

save_button_width = 200
save_button_height = 50
save_button_rect = (round(window_width*0.1) , round(window_height*0.5) , 200, 50) # (x, y, width, height)

retry_button_width = 200
retry_button_height = 50
retry_button_rect = (round(window_width*0.7) - round(retry_button_width*0.5), round(window_height*0.5) , 200, 50) # (x, y, width, height)



text_thickness = 2
text_color = (255, 255, 255) # white
text_font_face = cv2.FONT_HERSHEY_COMPLEX
text_font_scale = .78


def addButton(img, text, button_x, button_y, button_width, button_height, fill_color):
    (text_width, text_height), baseline = cv2.getTextSize(text, text_font_face,  text_font_scale, text_thickness )

    text_x_y = (button_x + round(button_width*0.5) - round(text_width*0.5), button_y + round(button_height*0.625) ) # bottom left corner

    cv2.rectangle(img, (button_x, button_y),
                    (button_x + button_width, button_y + button_height),
                    color=fill_color, thickness=-1)
    cv2.putText(img, text, text_x_y,
                text_font_face, text_font_scale, text_color, text_thickness)

def main_screen():
    main_img = app_background_img.copy()
    addButton(main_img, "Scan barcode", main_button_rect[0], main_button_rect[1], main_button_width, main_button_height, fill_color=(0,0,0))

    cv2.imshow('Barcode Scanner', main_img)
    cv2.setMouseCallback('Barcode Scanner', scan_barcode_button_callback)

#todo: make sure capture screen is same size
def confirmation_screen(barcode:str) -> None:

    confirmation_img = app_background_img.copy()

    (text_width, text_height), baseline = cv2.getTextSize(f"Detected barcode: {barcode}", text_font_face,  text_font_scale, text_thickness )
    text_x_y = (round(window_width*0.5) - round(text_width*0.5), round(window_height*0.3) ) # bottom left corner

    cv2.putText(confirmation_img, f"Detected barcode: {barcode}", text_x_y, text_font_face, text_font_scale, (0, 0, 0), text_thickness)

    addButton(confirmation_img, "Save barcode", save_button_rect[0], save_button_rect[1], save_button_width, save_button_height, fill_color=(0,100,0))
    addButton(confirmation_img, "Scan again", retry_button_rect[0], retry_button_rect[1], retry_button_width, retry_button_height, fill_color=(0,0,139))

    cv2.imshow('Barcode Scanner', confirmation_img)

def main():
    global current_screen

    main_screen()

    cv2.waitKey(0)
    cv2.destroyAllWindows()

main()