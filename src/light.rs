use crate::material::Material;
use crate::tuple::Tuple;
use std::f32::consts::SQRT_2;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Tuple,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Tuple) -> Self {
        assert!(position.is_point());
        PointLight {
            position,
            intensity,
        }
    }
}

pub fn lighting(
    material: Material,
    light: PointLight,
    point: Tuple,
    eye_vector: Tuple,
    normal_vector: Tuple,
) -> Tuple {
    // combine the surface color with the light's color/intensity
    let effective_color = material.color * light.intensity;

    // find the direction to the light source
    let light_vector = (light.position - point).normalize();

    // compute the ambient contribution
    let ambient = effective_color * material.ambient;

    // The value light_dot_normal represents the cosine of the angle between the light vector and the normal vector.
    // A negative number means the light is on the other side of the surface.
    let light_dot_normal = light_vector.dot(normal_vector);

    let diffuse;
    let specular;
    if light_dot_normal < 0.0 {
        diffuse = Tuple::color(0.0, 0.0, 0.0);
        specular = Tuple::color(0.0, 0.0, 0.0);
    } else {
        // compute the diffuse contribution
        diffuse = effective_color * material.diffuse * light_dot_normal;

        // The value reflect_dot_eye represents the cosine of the angle between the reflection vector and the eye vector.
        // A negative number means the light reflects away from the eye.
        let reflection_vector = (-light_vector).reflect(normal_vector);
        let reflection_dot_eye = reflection_vector.dot(eye_vector);

        if reflection_dot_eye <= 0.0 {
            specular = Tuple::color(0.0, 0.0, 0.0);
        } else {
            let factor = reflection_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    ambient + diffuse + specular
}

#[test]
fn test_a_point_light_has_a_position_and_intensity() {
    let intensity = Tuple::color(1.0, 1.0, 1.0);
    let position = Tuple::point(0.0, 0.0, 0.0);
    let light = PointLight::new(position, intensity);
    assert_eq!(light.position, position);
    assert_eq!(light.intensity, intensity);
}

#[test]
fn test_lighting_with_the_eye_between_the_light_and_the_surface() {
    let material = Material::new();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eye_vector = Tuple::vector(0.0, 0.0, -1.0);
    let normal_vector = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 0.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let result = lighting(material, light, position, eye_vector, normal_vector);
    // ambient + diffuse + specular
    // 0.1 + 0.9 + 0.9 = 1.9
    assert_eq!(result, Tuple::color(1.9, 1.9, 1.9));
}

#[test]
fn test_lighting_with_the_eye_between_light_and_surface_eye_offset_45_degrees()
{
    let material = Material::new();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eye_vector = Tuple::vector(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
    let normal_vector = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 0.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    // ambient + diffuse + no specular
    // 0.1 + 0.9 + 0.0 = 1.0
    let result = lighting(material, light, position, eye_vector, normal_vector);
    assert_eq!(result, Tuple::color(1.0, 1.0, 1.0));
}

#[test]
fn test_lighting_with_eye_opposite_surface_light_offset_45_degrees() {
    let material = Material::new();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eye_vector = Tuple::vector(0.0, 0.0, -1.0);
    let normal_vector = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 10.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let result = lighting(material, light, position, eye_vector, normal_vector);
    // ambient + partial diffuse + no specular
    // 0.1 + 0.9 * sqrt(2)/2.0 + 0 = 0.7364
    assert_eq!(result, Tuple::color(0.7364, 0.7364, 0.7364));
}

#[test]
fn test_lighting_with_eye_in_the_path_of_the_reflection_vector() {
    let material = Material::new();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eye_vector = Tuple::vector(0.0, -SQRT_2 / 2.0, -SQRT_2 / 2.0);
    let normal_vector = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 10.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let result = lighting(material, light, position, eye_vector, normal_vector);
    // ambient + partial diffuse + specular
    // 0.1 + 0.9 * sqrt(2)/2.0 + 0.9 = 1.63639
    assert_eq!(result, Tuple::color(1.63639, 1.63639, 1.63639));
}

#[test]
fn test_lighting_with_the_light_behind_the_surface() {
    let material = Material::new();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eye_vector = Tuple::vector(0.0, 0.0, -1.0);
    let normal_vector = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(
        Tuple::point(0.0, 0.0, 10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let result = lighting(material, light, position, eye_vector, normal_vector);
    // ambient + no diffuse + no specular
    // 0.1 + 0.0 + 0.0 = 0.1
    assert_eq!(result, Tuple::color(0.1, 0.1, 0.1));
}