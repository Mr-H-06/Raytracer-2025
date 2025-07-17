use super::triangle::Triangle;
use std::sync::Arc;

use super::Color;
use super::hittable;
use super::material::{Lambertian, Material};
use super::texture::ImageTexture;
use super::vec3::{Point3, Vec3};
use crate::hittable_list::HittableList;
use std::path::Path;
use tobj::LoadOptions;

pub fn load_model<M: Material + Clone + 'static>(
    file_path: &str,
    default_material: M,
) -> HittableList {
    println!("Loading model: {}", file_path);
    let mut models = HittableList::default();

    let (tobj_models, tobj_materials_res) = tobj::load_obj(
        file_path,
        &LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )
    .expect("Failed to load .obj file");

    let tobj_materials = tobj_materials_res.expect("Failed to load .mtl file");
    let mut materials: Vec<Arc<dyn Material>> = Vec::new();
    for mat in tobj_materials.clone() {
        let material: Arc<dyn Material> = if let Some(texture_filename) = mat.diffuse_texture {
            let base_path = Path::new(file_path)
                .parent()
                .unwrap_or_else(|| Path::new(""));
            let texture_path = base_path.join(texture_filename);

            let texture = ImageTexture::new(texture_path.to_str().unwrap());
            Arc::new(Lambertian::new_with_texture(texture))
        } else {
            let color = match mat.diffuse {
                Some(c) => Color::new(c[0] as f64, c[1] as f64, c[2] as f64),
                None => Color::new(0.8, 0.8, 0.8),
            };
            Arc::new(Lambertian::new(color))
        };
        materials.push(material);
    }
    for model in tobj_models {
        let mesh = &model.mesh;
        let positions = &mesh.positions;
        let normals = &mesh.normals;
        let texcoords = &mesh.texcoords;

        let has_normals = !normals.is_empty();
        let has_texcoords = !texcoords.is_empty();

        /*let material = match mesh.material_id {
            Some(id) => materials[id].clone(),
            None => Arc::new(default_material.clone()),
        };*/

        let default_normal = Vec3::new(0.0, 1.0, 0.0);
        //let default_uv = (0.0, 0.0);

        for i in (0..mesh.indices.len()).step_by(3) {
            let i0 = mesh.indices[i] as usize;
            let i1 = mesh.indices[i + 1] as usize;
            let i2 = mesh.indices[i + 2] as usize;

            let p0 = Point3::new(
                positions[3 * i0] as f64,
                positions[3 * i0 + 1] as f64,
                positions[3 * i0 + 2] as f64,
            );
            let p1 = Point3::new(
                positions[3 * i1] as f64,
                positions[3 * i1 + 1] as f64,
                positions[3 * i1 + 2] as f64,
            );
            let p2 = Point3::new(
                positions[3 * i2] as f64,
                positions[3 * i2 + 1] as f64,
                positions[3 * i2 + 2] as f64,
            );

            let n0 = if has_normals {
                Vec3::new(
                    normals[3 * i0] as f64,
                    normals[3 * i0 + 1] as f64,
                    normals[3 * i0 + 2] as f64,
                )
            } else {
                default_normal
            };
            let n1 = if has_normals {
                Vec3::new(
                    normals[3 * i1] as f64,
                    normals[3 * i1 + 1] as f64,
                    normals[3 * i1 + 2] as f64,
                )
            } else {
                default_normal
            };
            let n2 = if has_normals {
                Vec3::new(
                    normals[3 * i2] as f64,
                    normals[3 * i2 + 1] as f64,
                    normals[3 * i2 + 2] as f64,
                )
            } else {
                default_normal
            };

            let uv0 = if has_texcoords {
                (texcoords[2 * i0] as f64, 1.0 - texcoords[2 * i0 + 1] as f64)
            } else {
                (0.0, 0.0)
            };
            let uv1 = if has_texcoords {
                (texcoords[2 * i1] as f64, 1.0 - texcoords[2 * i1 + 1] as f64)
            } else {
                (0.0, 0.0)
            };
            let uv2 = if has_texcoords {
                (texcoords[2 * i2] as f64, 1.0 - texcoords[2 * i2 + 1] as f64)
            } else {
                (0.0, 0.0)
            };
            let triangle_to_add: Arc<dyn hittable::Hittable>;

            // --- 核心的分发逻辑在这里 ---
            match mesh.material_id {
                Some(id) => {
                    // 根据ID获取tobj的材质信息
                    let mat_info = &tobj_materials[id];

                    if let Some(texture_filename) = &mat_info.diffuse_texture {
                        // 情况1：有漫反射纹理，创建 Lambertian<ImageTexture>
                        let base_path = Path::new(file_path)
                            .parent()
                            .unwrap_or_else(|| Path::new(""));
                        let texture_path = base_path.join(texture_filename);
                        let texture = ImageTexture::new(texture_path.to_str().unwrap());

                        let concrete_material = Lambertian::new_with_texture(texture);

                        triangle_to_add = Arc::new(Triangle::new(
                            p0,
                            p1,
                            p2,
                            n0,
                            n1,
                            n2,
                            uv0,
                            uv1,
                            uv2,
                            concrete_material,
                        ));
                    } else {
                        // 情况2：没有纹理，使用漫反射颜色，创建 Lambertian<SolidColor>
                        let color = mat_info.diffuse.map_or(Color::new(0.8, 0.8, 0.8), |c| {
                            Color::new(c[0] as f64, c[1] as f64, c[2] as f64)
                        });

                        let concrete_material = Lambertian::new(color);

                        triangle_to_add = Arc::new(Triangle::new(
                            p0,
                            p1,
                            p2,
                            n0,
                            n1,
                            n2,
                            uv0,
                            uv1,
                            uv2,
                            concrete_material,
                        ));
                    }
                }
                None => {
                    triangle_to_add = Arc::new(Triangle::new(
                        p0,
                        p1,
                        p2,
                        n0,
                        n1,
                        n2,
                        uv0,
                        uv1,
                        uv2,
                        default_material.clone(),
                    ));
                }
            }

            models.add(triangle_to_add);
        }
    }
    println!("Model loaded with {} triangles.", models.objects.len());
    models
}
