use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

use crate::hitbox::HitBox;

#[derive(Clone)]
#[pyclass(subclass, module = "arcade.sprite.base")]
pub struct BasicSprite {
    texture: PyObject,
    position: (f32, f32),
    depth: f32,
    scale: (f32, f32),
    width: f32,
    height: f32,
    hitbox: HitBox,
    color: (u8, u8, u8, u8),
    sprite_lists: Vec<PyObject>,
    angle: f32,
}

impl IntoPy<Py<PyTuple>> for BasicSprite {
    fn into_py(self, py: Python) -> Py<PyTuple> {
        let vec: Vec<PyObject> = vec![self.into_py(py)];
        PyTuple::new(py, vec).into()
    }
}

#[pymethods]
impl BasicSprite {
    #[new]
    #[pyo3(signature = (texture, scale=1.0, center_x=0.0, center_y=0.0, **_kwargs))]
    fn new(
        py: Python<'_>,
        texture: PyObject,
        scale: Option<f32>,
        center_x: Option<f32>,
        center_y: Option<f32>,
        _kwargs: Option<&PyDict>,
    ) -> Self {
        let final_scale = (scale.unwrap_or(1.0), scale.unwrap_or(1.0));
        let final_position = (center_x.unwrap_or(0.0), center_y.unwrap_or(0.0));
        let points: Vec<(f32, f32)> = texture
            .getattr(py, "hit_box_points")
            .unwrap()
            .extract(py)
            .expect("Failed to Load Hit Box Points from Texture");
        let mut width: f32 = texture
            .getattr(py, "width")
            .unwrap()
            .extract(py)
            .expect("Failed to Load Width from Texture");
        let mut height: f32 = texture
            .getattr(py, "height")
            .unwrap()
            .extract(py)
            .expect("Failed to Load Height From Texture");
        width *= final_scale.0;
        height *= final_scale.1;

        BasicSprite {
            texture,
            scale: final_scale,
            position: final_position,
            depth: 0.0,
            width,
            height,
            color: (255, 255, 255, 255),
            hitbox: HitBox {
                points,
                position: final_position,
                scale: final_scale,
            },
            sprite_lists: Vec::new(),
            angle: 0.0,
        }
    }

    #[getter]
    fn get_position(&self) -> PyResult<(f32, f32)> {
        Ok(self.position)
    }

    #[setter]
    fn set_position(&mut self, py: Python<'_>, new_value: (f32, f32)) -> PyResult<()> {
        if new_value == self.position {
            return Ok(());
        }

        self.position = new_value;
        self.hitbox.position = new_value;

        for sprite_list in self.sprite_lists.iter() {
            sprite_list.call_method1(py, "_update_position", self.clone())?;
        }

        Ok(())
    }

    #[getter]
    fn get_scale(&self) -> PyResult<f32> {
        Ok(self.scale.0)
    }

    #[setter]
    fn set_scale(&mut self, py: Python<'_>, new_value: f32) -> PyResult<()> {
        self.set_scalexy(py, (new_value, new_value))
    }

    #[getter]
    fn get_scalexy(&self) -> PyResult<(f32, f32)> {
        Ok(self.scale)
    }

    #[setter]
    fn set_scalexy(&mut self, py: Python<'_>, new_value: (f32, f32)) -> PyResult<()> {
        if new_value == self.scale {
            return Ok(());
        }

        self.scale = new_value;
        self.hitbox.scale = new_value;

        let tex_width: f32 = self
            .texture
            .getattr(py, "width")
            .unwrap()
            .extract(py)
            .expect("Failed to Load Width From Texture");

        let tex_height: f32 = self
            .texture
            .getattr(py, "height")
            .unwrap()
            .extract(py)
            .expect("Failed to Load Height From Texture");

        self.width = tex_width * self.scale.0;
        self.height = tex_height * self.scale.1;

        for sprite_list in self.sprite_lists.iter() {
            sprite_list.call_method1(py, "_update_size", self.clone())?;
        }

        Ok(())
    }

    #[getter]
    fn get_depth(&self) -> PyResult<f32> {
        Ok(self.depth)
    }

    #[setter]
    fn set_depth(&mut self, py: Python<'_>, new_value: f32) -> PyResult<()> {
        if new_value == self.depth {
            return Ok(());
        }

        self.depth = new_value;

        for sprite_list in self.sprite_lists.iter() {
            sprite_list.call_method1(py, "_update_depth", self.clone())?;
        }

        Ok(())
    }

    #[getter]
    fn get_texture(&self) -> PyResult<PyObject> {
        Ok(self.clone().texture)
    }

    #[setter]
    fn set_texture(&mut self, py: Python<'_>, new_value: PyObject) -> PyResult<()> {
        if new_value.is(&self.texture) {
            return Ok(());
        }

        self.texture = new_value;

        let new_width: f32 = self
            .texture
            .getattr(py, "width")
            .unwrap()
            .extract(py)
            .expect("Failed to Load Width from Texture");
        self.width = new_width * self.scale.0;

        let new_height: f32 = self
            .texture
            .getattr(py, "height")
            .unwrap()
            .extract(py)
            .expect("Failed to Load Height From Texture");
        self.height = new_height * self.scale.1;

        for sprite_list in self.sprite_lists.iter() {
            sprite_list.call_method1(py, "_update_texture", self.clone())?;
        }

        Ok(())
    }

    #[getter]
    fn get_color(&self) -> PyResult<(u8, u8, u8, u8)> {
        Ok(self.color)
    }

    #[setter]
    fn set_color(&mut self, py: Python<'_>, new_value: Vec<u8>) -> PyResult<()> {
        let new_color: (u8, u8, u8, u8) = match new_value.len() {
            4 => (new_value[0], new_value[1], new_value[2], new_value[3]),
            3 => (new_value[0], new_value[1], new_value[2], self.color.3),
            _ => panic!("Color must be 3 or 4 ints from 0-255"),
        };

        if new_color == self.color {
            return Ok(());
        }

        self.color = new_color;

        for sprite_list in self.sprite_lists.iter() {
            sprite_list.call_method1(py, "_update_color", self.clone())?;
        }

        Ok(())
    }

    #[getter]
    fn get_alpha(&self) -> PyResult<u8> {
        Ok(self.color.3)
    }

    #[setter]
    fn set_alpha(&mut self, py: Python<'_>, new_value: u8) -> PyResult<()> {
        if self.color.3 == new_value {
            return Ok(());
        }

        self.color.3 = new_value;

        for sprite_list in self.sprite_lists.iter() {
            sprite_list.call_method1(py, "_update_color", self.clone())?;
        }

        Ok(())
    }

    #[getter]
    fn get_visible(&self) -> PyResult<bool> {
        Ok(self.color.3 > 0)
    }

    #[setter]
    fn set_visible(&mut self, py: Python<'_>, new_value: bool) -> PyResult<()> {
        self.color.3 = if new_value { 255 } else { 0 };

        for sprite_list in self.sprite_lists.iter() {
            sprite_list.call_method1(py, "_update_color", self.clone())?;
        }

        Ok(())
    }

    #[getter]
    fn get_width(&self) -> PyResult<f32> {
        Ok(self.width)
    }

    #[setter]
    fn set_width(&mut self, py: Python<'_>, new_value: f32) -> PyResult<()> {
        if self.width == new_value {
            return Ok(());
        }

        let tex_width: f32 = self
            .texture
            .getattr(py, "width")
            .unwrap()
            .extract(py)
            .expect("Failed to Load Width From Texture");

        self.scale = (new_value / tex_width, self.scale.1);
        self.width = new_value;
        self.hitbox.scale = self.scale;

        for sprite_list in self.sprite_lists.iter() {
            sprite_list.call_method1(py, "_update_width", self.clone())?;
        }

        Ok(())
    }

    #[getter]
    fn get_height(&self) -> PyResult<f32> {
        Ok(self.height)
    }

    #[setter]
    fn set_height(&mut self, py: Python<'_>, new_value: f32) -> PyResult<()> {
        if self.height == new_value {
            return Ok(());
        }

        let tex_height: f32 = self
            .texture
            .getattr(py, "height")
            .unwrap()
            .extract(py)
            .expect("Failed to Load Height From Texture");

        self.scale = (self.scale.0, new_value / tex_height);
        self.height = new_value;
        self.hitbox.scale = self.scale;

        for sprite_list in self.sprite_lists.iter() {
            sprite_list.call_method1(py, "_update_height", self.clone())?;
        }

        Ok(())
    }

    #[getter]
    fn get_center_x(&self) -> PyResult<f32> {
        Ok(self.position.0)
    }

    #[setter]
    fn set_center_x(&mut self, py: Python<'_>, new_value: f32) -> PyResult<()> {
        self.set_position(py, (new_value, self.position.1))
    }

    #[getter]
    fn get_center_y(&self) -> PyResult<f32> {
        Ok(self.position.1)
    }

    #[setter]
    fn set_center_y(&mut self, py: Python<'_>, new_value: f32) -> PyResult<()> {
        self.set_position(py, (self.position.0, new_value))
    }

    #[getter]
    fn get_left(&self) -> PyResult<f32> {
        Ok(self.hitbox.left_native())
    }

    #[setter]
    fn set_left(&mut self, py: Python<'_>, new_value: f32) -> PyResult<()> {
        let leftmost = self.hitbox.left_native();
        let diff = new_value - leftmost;
        self.set_center_x(py, self.position.0 + diff)
    }

    #[getter]
    fn get_right(&self) -> PyResult<f32> {
        Ok(self.hitbox.right_native())
    }

    #[setter]
    fn set_right(&mut self, py: Python<'_>, new_value: f32) -> PyResult<()> {
        let rightmost = self.hitbox.right_native();
        let diff = rightmost - new_value;
        self.set_center_x(py, self.position.0 - diff)
    }

    #[getter]
    fn get_bottom(&self) -> PyResult<f32> {
        Ok(self.hitbox.bottom_native())
    }

    #[setter]
    fn set_bottom(&mut self, py: Python<'_>, new_value: f32) -> PyResult<()> {
        let lowest = self.hitbox.bottom_native();
        let diff = lowest - new_value;
        self.set_center_y(py, self.position.1 - diff)
    }

    #[getter]
    fn get_top(&self) -> PyResult<f32> {
        Ok(self.hitbox.top_native())
    }

    #[setter]
    fn set_top(&mut self, py: Python<'_>, new_value: f32) -> PyResult<()> {
        let highest = self.hitbox.top_native();
        let diff = highest - new_value;
        self.set_center_y(py, self.position.1 - diff)
    }

    fn update_spatial_hash(&self, py: Python<'_>) -> PyResult<()> {
        for sprite_list in self.sprite_lists.iter() {
            let spatial_hash: PyObject = sprite_list
                .getattr(py, "spatial_hash")
                .unwrap()
                .extract(py)
                .unwrap();

            if spatial_hash.is(&py.None()) {
                return Ok(());
            }

            spatial_hash.call_method1(py, "move", self.clone())?;
        }

        Ok(())
    }

    fn register_sprite_list(&mut self, new_list: PyObject) {
        self.sprite_lists.push(new_list);
    }

    fn remove_from_sprite_lists(&mut self, py: Python<'_>) -> PyResult<()> {
        while !self.sprite_lists.is_empty() {
            self.sprite_lists[0].call_method1(py, "remove", self.clone())?;
        }

        self.sprite_lists.clear();

        Ok(())
    }

    fn update(&self) {}
}

#[derive(FromPyObject)]
enum PathOrTexture {
    First(String),
    Second(PyObject),
}

#[pyclass(subclass, extends=BasicSprite, module="arcade.sprite.sprite")]
pub struct Sprite {}

#[pymethods]
impl Sprite {
    #[new]
    #[pyo3(signature = (path_or_texture, scale=1.0, center_x=0.0, center_y=0.0, angle=0.0, **_kwargs))]
    fn new(
        py: Python<'_>,
        path_or_texture: PathOrTexture,
        scale: Option<f32>,
        center_x: Option<f32>,
        center_y: Option<f32>,
        angle: Option<f32>,
        _kwargs: Option<&PyDict>,
    ) -> (Self, BasicSprite) {
        let texture: PyObject = match path_or_texture {
            PathOrTexture::First(path_string) => {
                let arcade = PyModule::import(py, "arcade").expect("Failed to import arcade");
                arcade
                    .getattr("load_texture")
                    .expect("No arcade.load_texture function found")
                    .call1(PyTuple::new(py, vec![path_string]))
                    .expect("Failed to execute arcade.load_texture")
                    .extract()
                    .expect("Failed to extract PyObject from arcade.load_texture")
            }
            PathOrTexture::Second(object) => {
                let cls: &str = object.clone().into_ref(py).get_type().name().unwrap();
                let final_object: PyObject = match cls {
                    "Texture" => object,
                    "Path" => panic!("Handle pathlib here"),
                    _ => panic!("Unknown Type Passed to sprite constructor"),
                };
                final_object
            }
        };
        let mut basic = BasicSprite::new(py, texture, scale, center_x, center_y, _kwargs);
        basic.angle = angle.unwrap_or(0.0);
        (Self {}, basic)
    }
}
