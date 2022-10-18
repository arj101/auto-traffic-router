from cmath import inf
import sys
import cv2
import numpy as np
import time
import math


def dist(x1, y1, x2, y2):
    math.sqrt(math.pow(x2 - x1, 2) + math.pow(y2 - y1, 2))


upper_h = 10
upper_s = 255
upper_v = 255

lower_h = 0
lower_s = 70
lower_v = 50


def constrain(val, l, u):
    if val > u:
        return u
    if val < l:
        return l
    return val


roadmask_l = cv2.cvtColor(cv2.imread(
    '/home/ge9x/science-fair-2022/road-masks/1-4-l.png'), cv2.COLOR_BGR2GRAY)


kernel = np.array([
    [4, 4, 4, 4, 4],
    [3, 4, 4, 4, 3],
    [2, 3, 4, 3, 2],
    [1, 2, 3, 2, 1],
    [1, 2, 3, 2, 1],
], dtype=np.uint8)

cam = cv2.VideoCapture(-1)

if not cam.isOpened():
    print('cannot open camera')
    sys.exit()

# tracker = object_tracker.ObjectTracker(10, 0.05)
data = {'e': ((100, 100), 3, 0, 0)}
buffer = {}
thresh_dist = 15
thresh_time = 1.0
counter = 0

while True:
    ret, frame = cam.read()
    if not ret:
        print('error reading frame')
        continue

    blurred = cv2.GaussianBlur(frame, (7, 7), 0)
    cv2.imshow('blurred', blurred)
    opened = cv2.morphologyEx(blurred, cv2.MORPH_CLOSE, kernel)
    thresh_lower = np.array([lower_h, lower_s, lower_v])
    thresh_upper = np.array([upper_h, upper_s, upper_v])
    hsv = cv2.cvtColor(opened, cv2.COLOR_BGR2HSV)
    mask1 = cv2.inRange(hsv, np.array([0, 70, 50]), np.array([10, 255, 255]))
    mask2 = cv2.inRange(hsv, np.array(
        [170, 70, 50]), np.array([180, 255, 255]))

    thresh = mask1 | mask2

    cv2.imshow('result', thresh)
    cv2.imshow('processed', opened)

    contours, _ = cv2.findContours(
        thresh, cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)
    img = frame

    for cnt in contours:
        area = cv2.contourArea(cnt)

        if (area > 20 and area < 500):
            approx = cv2.approxPolyDP(
                cnt, 0.1 * cv2.arcLength(cnt, True), True)
            if len(approx) == 4:
                color = (255, 255, 0)
                rect = cv2.minAreaRect(cnt)
                box = cv2.boxPoints(rect)
                box = np.int0(box)
                cx = 0
                cy = 0
                for (x, y) in box:
                    cx += x
                    cy += y
                cx /= 4
                cy /= 4
                cv2.circle(img, (int(cx), int(cy)), 2, (0, 255, 0), 2)
                found = False
                nearest_dist = inf
                nearest_key = None
                nearest_vel = None
                nearest_angle = None
                s = ''
                for (key, ((x2, y2), t, vel, angle)) in data.items():

                    d = math.dist([float(cx), float(cy)],
                                  [float(x2), float(y2)])
                    # print(float(cx))

                    max_d = math.dist(
                        [0, 0],
                        [(thresh_dist + vel) * math.cos(angle),
                         (thresh_dist + vel) * math.sin(angle)]
                    )

                    if d <= constrain(max_d, thresh_dist, 100):
                        if (d < nearest_dist):
                            dt = time.monotonic() - t
                            vel = d/(5 * dt)
                            angle = math.atan2(cy - y2, cx - x2)
                            found = True
                            nearest_key = key
                            nearest_dist = d
                            nearest_angle = angle
                            nearest_vel = vel
                if not found:
                    data[counter] = ((cx, cy), time.monotonic(), 0, 0)
                    counter += 1
                else:
                    data[nearest_key] = (
                        (cx, cy), time.monotonic(), nearest_vel, nearest_angle)
                print(s)

                # cv2.drawContours(img, [box], 0, color, 3)
            # cv2.drawContours(img, [approx], 0, (255,0,255), 1)
    # for ((x, y), _) in tracker.get_data().values():
    #     cv2.circle(img, (int(x), int(y)), 2, (255, 255, 0), 4)

    to_remove = []
    s = ''
    for (key, ((x, y), t, vel, angle)) in data.items():
        s += str(key) + ',' + str(x) + ',' + str(y) + ',' + str(vel) + ' '
        if time.monotonic() - t >= thresh_time:
            to_remove.append(key)
        else:

            cv2.putText(img, str(key), (int(x), int(y)),
                        cv2.FONT_HERSHEY_SIMPLEX, 1, (0, 0, 0), 2, cv2.LINE_AA)

            for i in range(-100, 100):
                theta = -math.pi * float(i)/100
                r = thresh_dist + (math.pi - abs(theta))/math.pi * vel
                cv2.circle(img, (int(x + r * math.cos(theta + angle)), int(y + r * math.sin(theta + angle))),
                           1, (25, 25, 255), 1)

    for e in to_remove:
        data.pop(e)

    print(s)
    sys.stdout.flush()

    cv2.imshow('img', img)
    # v = cv2.split(thresh)
    cv2.imshow('masked', cv2.bitwise_and(thresh, thresh, mask=roadmask_l))
    cv2.imshow('masked1', cv2.bitwise_or(
        cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY), roadmask_l))

    if cv2.waitKey(1) == ord('q'):
        break


cam.release()
cv2.destroyAllWindows()
