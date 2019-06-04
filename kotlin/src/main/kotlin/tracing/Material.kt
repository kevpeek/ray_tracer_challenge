package tracing

import display.Color
import geometry.Point
import geometry.Vector

fun lighting(material: Material, light: PointLight, position: Point, eyeVector: Vector, normal: Vector): Color {
    val ambient = ambientContribution(material, light)
    val diffuse = diffuseContribution(material, light, position, normal)
    val specular = specularContribution(material, light, position, eyeVector, normal)
    return ambient + diffuse + specular
}

private fun ambientContribution(material: Material, light: PointLight): Color {
    val effectiveColor = effectiveColor(material, light)
    return effectiveColor * material.ambient
}

private fun diffuseContribution(material: Material, light: PointLight, position: Point, normal: Vector): Color {
    val lightDirection = (light.position - position).normalize()
    val lightDotNormal = lightDirection.dot(normal)
    return if (lightDotNormal < 0) {
        Color.BLACK
    } else {
        effectiveColor(material, light) * material.diffuse * lightDotNormal
    }
}

private fun effectiveColor(material: Material, light: PointLight): Color {
    return material.color * light.intensity
}

private fun specularContribution(material: Material, light: PointLight, position: Point, eyeVector: Vector, normal: Vector): Color {
    val lightDirection = (light.position - position).normalize()
    val lightDotNormal = lightDirection.dot(normal)
    return if (lightDotNormal < 0) {
        Color.BLACK
    } else {
        val reflectVector = -lightDirection.reflect(normal)
        val reflectDotEye = reflectVector.dot(eyeVector)

        if (reflectDotEye < 0) {
            Color.BLACK
        } else {
            val factor = Math.pow(reflectDotEye, material.shininess)
            light.intensity * material.specular * factor
        }
    }
}

class Material(
    val color: Color = Color.WHITE,
    val ambient: Double = 0.1,
    val diffuse: Double = 0.9,
    val specular: Double = 0.9,
    val shininess: Double = 200.0
) {

    companion object {
        val DEFAULT = Material()
    }
}