$fn = 100;

module ring(radius, width) {
    difference() {
        circle(r = radius);
        circle(r = radius - width);
    }
}

module pizza_slice(radius, angle) {
    if (angle > 90) {
        union() {
            pizza_slice(radius, 90);
            rotate(90) pizza_slice(radius, angle - 90);
        }
    } else {
        intersection() {
            circle(r = radius);
            polygon([
                [0, 0],
                [radius * 2, 0],
                [radius * 2 * cos(angle), radius * 2 * sin(angle)]
            ]);
        }
    }
}

module segments(n, radius, width) {
    let (angle = 360 / (n * 2)) {
        for (i = [1 : n]) {
            rotate((i - 1) * 2 * angle) intersection() {
                pizza_slice(radius, angle);
                ring(radius, width);
            }
        }
    }
}

module segment_rings(n, width, spacing, min_radius) {
    for (i = [1 : n]) {
        segments(pow(2, i - 1), min_radius + ((i - 1) * (width + spacing)), width);
    }
}

module disc() {
    linear_extrude(height = 2) {
        difference() {
            circle(r = 42);
            circle(r = 6);
            segment_rings(8, 0.8, 3, 13);
        }
    }
}

disc();
