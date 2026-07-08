struct Uniforms {
    bounds_size: vec2<f32>,
    radius: f32,
    stroke_width: f32,
    arc_start_radians: f32,
    sweep_radians: f32,
    pixel_size: f32,
    aa_scale: f32,
    track: vec4<f32>,
    fg: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> u: Uniforms;

struct VsOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VsOut {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0)
    );

    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, 0.0)
    );

    var out: VsOut;
    out.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    out.uv = uvs[vertex_index];

    return out;
}

const TAU: f32 = 6.28318530718;

fn wrap_angle(angle: f32) -> f32 {
    return angle - floor(angle / TAU) * TAU;
}

fn ring_alpha(d: f32, radius: f32, width: f32, aa: f32) -> f32 {
    let half_width = width * 0.5;

    let outer = 1.0 - smoothstep(
        radius + half_width - aa,
        radius + half_width + aa,
        d
    );

    let inner = smoothstep(
        radius - half_width - aa,
        radius - half_width + aa,
        d
    );

    return outer * inner;
}

fn disk_alpha(p: vec2<f32>, center: vec2<f32>, radius: f32, aa: f32) -> f32 {
    return 1.0 - smoothstep(
        radius - aa,
        radius + aa,
        length(p - center)
    );
}

fn premultiply(color: vec4<f32>, coverage: f32) -> vec4<f32> {
    let a = color.a * coverage;
    return vec4<f32>(color.rgb * a, a);
}

fn over(src: vec4<f32>, dst: vec4<f32>) -> vec4<f32> {
    return src + dst * (1.0 - src.a);
}

@fragment
fn fs_main(input: VsOut) -> @location(0) vec4<f32> {
    let p = input.uv * u.bounds_size;
    let center = u.bounds_size * 0.5;
    let q = p - center;

    let d = length(q);
    let aa = max(fwidth(d), u.pixel_size) * u.aa_scale;

    let track_coverage = ring_alpha(
        d,
        u.radius,
        u.stroke_width,
        aa
    );

    let angle = wrap_angle(atan2(q.y, q.x) - u.arc_start_radians);
    var body_angle: f32 = 0.0;

    if angle <= u.sweep_radians {
        body_angle = 1.0;
    }

    let body_coverage = track_coverage * body_angle;

    let half_width = u.stroke_width * 0.5;

    let start_cap_center = center + vec2<f32>(
        cos(u.arc_start_radians),
        sin(u.arc_start_radians)
    ) * u.radius;

    let end_rotation = u.arc_start_radians + u.sweep_radians;
    let end_cap_center = center + vec2<f32>(
        cos(end_rotation),
        sin(end_rotation)
    ) * u.radius;

    let start_cap = disk_alpha(p, start_cap_center, half_width, aa);
    let end_cap = disk_alpha(p, end_cap_center, half_width, aa);

    let fg_coverage = max(body_coverage, max(start_cap, end_cap));

    let track = premultiply(u.track, track_coverage);
    let fg = premultiply(u.fg, fg_coverage);

    return over(fg, track);
}
