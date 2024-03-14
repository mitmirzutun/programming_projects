import math
import multiprocessing
import turtle
import typing


def jump(turtle_obj: turtle.Turtle, vector: tuple[float, float], relative: bool = True, polar: bool = True) -> None:
    pendown = turtle_obj.pen()["pendown"]
    turtle_obj.penup()
    if relative:
        if polar:
            radius, phi = vector
            turtle_obj.goto(turtle_obj.xcor() + math.cos(phi + turtle_obj.heading()) * radius,
                            turtle_obj.ycor() + math.sin(phi + turtle_obj.heading()) * radius)
        else:
            turtle_obj.goto(turtle_obj.xcor() + vector[0], turtle_obj.ycor() + vector[1])
    else:
        if polar:
            radius, phi = vector
            turtle_obj.goto(math.cos(phi) * radius, math.sin(phi) * radius)
        else:
            turtle_obj.goto(vector[0], vector[1])
    if pendown:
        turtle_obj.pendown()


class AnalogClock:
    def __init__(self, turtle_obj: typing.Optional[turtle.Turtle] = None, pos: tuple[float, float] = (0, 0),
                 radius: float = 100, polar: bool = True):
        if polar:
            x = math.cos(pos[1]) * pos[0]
            y = math.sin(pos[1]) * pos[0]
        else:
            x, y = pos
        self.turtle: turtle.Turtle
        if turtle_obj is None:
            self.turtle = turtle.Turtle()
        else:
            self.turtle = turtle_obj
        # self.turtle.hideturtle()
        self.turtle.radians()
        turtle.tracer(0)
        self.pos: tuple[float, float] = pos
        self.radius: float = radius
        self.__hour_strokes: list[tuple[float, float, float]] = []
        self.__5_minute_strokes: list[tuple[float, float, float]] = []
        for hour_angle in range(12):
            self.__hour_strokes.append((math.pi * hour_angle / 6, x + math.cos(math.pi * hour_angle / 6) * .9 * radius,
                                        y + math.sin(math.pi * hour_angle / 6) * .9 * radius))
            for minute_angle in range(4):
                self.__5_minute_strokes.append(
                    ((hour_angle * 30 + minute_angle * 6 + 6) * math.pi / 180,
                     x + math.cos((hour_angle * 30 + minute_angle * 6 + 6) * math.pi / 180) * .95 * radius,
                     y + math.sin((hour_angle * 30 + minute_angle * 6 + 6) * math.pi / 180) * .95 * radius))

    @property
    def radius(self) -> float:
        return self.__radius

    @radius.setter
    def radius(self, value: float) -> None:
        if value <= 0:
            raise ValueError(f"Expected radius to be greater than zero. Gor {value}")
        self.__radius = value

    @property
    def pos(self) -> tuple[float, float]:
        return self.__pos

    @pos.setter
    def pos(self, value: tuple[float, float]) -> None:
        self.__pos = value

    def run(self):
        while True:
            self.update()

    def update(self):
        import time
        timestamp=time.time()
        seconds=timestamp%60
        minutes=(timestamp//60)%60
        hours=(timestamp//3600)%12
        self.turtle.clear()
        for hour_angle, x_stroke, y_stroke in self.__hour_strokes:
            stroke = (x_stroke, y_stroke)
            jump(self.turtle, stroke, relative=False, polar=False)
            self.turtle.setheading(hour_angle)
            self.turtle.forward(self.radius * .1)
        for minute_angle, x_stroke, y_stroke in self.__5_minute_strokes:
            stroke = (x_stroke, y_stroke)
            jump(self.turtle, stroke, relative=False, polar=False)
            self.turtle.setheading(minute_angle)
            self.turtle.forward(self.radius * .05)
        jump(self.turtle, self.pos, relative=False)
        self.turtle.pensize(5)
        self.turtle.setheading(math.pi/2-math.pi*hours/6)
        self.turtle.forward(0.4*self.radius)
        self.turtle.pensize(1)
        jump(self.turtle, self.pos, relative=False)
        self.turtle.setheading(math.pi/2-math.pi*minutes/30)
        self.turtle.forward(0.85*self.radius)
        jump(self.turtle, self.pos, relative=False)
        self.turtle.pencolor("#ff0000")
        self.turtle.setheading(math.pi/2-math.pi*seconds/30)
        self.turtle.forward(0.85*self.radius)
        self.turtle.pencolor("#000000")
        turtle.update()

class DigitalClock:
    def __init__(self):
        pass
