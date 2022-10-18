import math
import time


def dist(x1, y1, x2, y2):
    math.sqrt(math.pow(x2 - x1, 2) + math.pow(y2 - y1, 2))


class ObjectTracker:
    def __init__(self, thresh_dist, thresh_time):
        self.data = {}
        self.thresh_dist = thresh_dist
        self.thresh_time = thresh_time
        self.count = 0

    @classmethod
    def on_position(self, x, y):
        found = False
        for (key, ((x2, y2), t)) in self.data.items():
            d = dist(x, y, x2, y2)
            if d <= self.thresh_dist:
                self.data[key] = ((x, y), time.monotonic())
                found = True
        if not found:
            self.data.set(self.count, ((x, y), time.monotonic()))
            self.count += 1

    @classmethod
    def update(self):
        for (key, ((x, y), t)) in self.data.items():
            if time.monotonic() - t >= self.thresh_time:
                self.data.pop(key)
