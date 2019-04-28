package tracing

import display.Color
import geometry.Point
import geometry.Vector

fun lighting(material: Material, light: PointLight, position: Point, eyeVector: Vector, normal: Vector): Color {
    val ambient = ambientContribution(material, light)
    val diffuse = diffuseContribution(material, light, position, normal)
    val specular = specularContribution(material, light, position, eyeVector, normal)
    println("ambient: $ambient, diffuse: $diffuse, specular: $specular")
    return ambient + diffuse + specular
}

private fun ambientContribution(material: Material, light: PointLight): Color {
    val effectiveColor = material.color * light.intensity
    return effectiveColor * material.ambient
}

private fun diffuseContribution(material: Material, light: PointLight, position: Point, normal: Vector): Color {
    val effectiveColor = material.color * light.intensity
    val lightDirection = (light.position - position).normalize()
    val lightDotNormal = lightDirection.dot(normal)
    return if (lightDotNormal < 0) {
        Color.BLACK
    } else {
        effectiveColor * material.diffuse * lightDotNormal
    }
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

class Material {
    val color = Color(1, 1, 1)
    val ambient = 0.1
    val diffuse = 0.9
    val specular = 0.9
    val shininess = 200.0

    companion object {
        val DEFAULT = Material()
    }
}