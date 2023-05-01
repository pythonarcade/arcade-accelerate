use pyo3::{prelude::*, intern};
use pyo3::types::{PyDict, PyTuple};

use crate::hitbox::HitBox;

#[pyclass(subclass, module = "arcade.sprite.base")]
pub struct BasicSprite {
    #[pyo3(get, name="_texture")]
    texture: PyObject,
    position: (f32, f32),
    #[pyo3(get, name = "_depth")]
    depth: f32,
    #[pyo3(get, name = "_scale")]
    scale: (f32, f32),
    #[pyo3(get, name = "_width")]
    width: f32,
    #[pyo3(get, name = "_height")]
    height: f32,
    hitbox: HitBox,
    #[pyo3(get, name = "_color")]
    color: (u8, u8, u8, u8),
    sprite_lists: Vec<PyObject>,
    #[pyo3(get, name = "_angle")]
    angle: f32,
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
            .getattr(py, intern!(py, "hit_box_points"))
            .unwrap()
            .extract(py)
            .expect("Failed to Load Hit Box Points from Texture");
        let mut width: f32 = texture
            .getattr(py, intern!(py, "width"))
            .unwrap()
            .extract(py)
            .expect("Failed to Load Width from Texture");
        let mut height: f32 = texture
            .getattr(py, intern!(py, "height"))
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
                angle: 0.0,
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
    fn set_position(mut self_: PyRefMut<'_, BasicSprite>, py: Python<'_>, new_value: (f32, f32)) -> PyResult<()> {
        if new_value == self_.position {
            return Ok(());
        }

        self_.position = new_value;
        self_.hitbox.position = new_value;

        let sprite_lists = self_.sprite_lists.clone();
        let s= Py::from(self_);
        for sprite_list in sprite_lists.iter() {
            sprite_list.call_method1(py, intern!(py, "_update_position"), (&s,))?;
        }

        Ok(())
    }

    #[getter]
    fn get_scale(&self) -> PyResult<f32> {
        Ok(self.scale.0)
    }

    #[setter]
    fn set_scale(self_: PyRefMut<BasicSprite>, py: Python<'_>, new_value: f32) -> PyResult<()> {
        BasicSprite::set_scalexy(self_, py, (new_value, new_value))
    }

    #[getter]
    fn get_scalexy(&self) -> PyResult<(f32, f32)> {
        Ok(self.scale)
    }

    #[setter]
    fn set_scalexy(mut self_: PyRefMut<BasicSprite>, py: Python<'_>, new_value: (f32, f32)) -> PyResult<()> {
        if new_value == self_.scale {
            return Ok(());
        }

        self_.scale = new_value;
        self_.hitbox.scale = new_value;

        let tex_width: f32 = self_
            .texture
            .getattr(py, intern!(py, "width"))
            .unwrap()
            .extract(py)
            .expect("Failed to Load Width From Texture");

        let tex_height: f32 = self_
            .texture
            .getattr(py, intern!(py, "height"))
            .unwrap()
            .extract(py)
            .expect("Failed to Load Height From Texture");

        self_.width = tex_width * self_.scale.0;
        self_.height = tex_height * self_.scale.1;

        for sprite_list in self_.sprite_lists.iter() {
            sprite_list.call_method1(py, intern!(py, "_update_size"), (&self_,))?;
        }

        Ok(())
    }

    #[getter]
    fn get_depth(&self) -> PyResult<f32> {
        Ok(self.depth)
    }

    #[setter]
    fn set_depth(mut self_: PyRefMut<BasicSprite>, py: Python<'_>, new_value: f32) -> PyResult<()> {
        if new_value == self_.depth {
            return Ok(());
        }

        self_.depth = new_value;

        for sprite_list in self_.sprite_lists.iter() {
            sprite_list.call_method1(py, intern!(py, "_update_depth"), (&self_,))?;
        }

        Ok(())
    }

    #[getter]
    fn get_texture(&self, py: Python<'_>) -> PyResult<PyObject> {
        Ok(self.texture.clone_ref(py))
    }

    #[setter]
    fn set_texture(mut self_: PyRefMut<BasicSprite>, py: Python<'_>, new_value: PyObject) -> PyResult<()> {
        if new_value.is(&self_.texture) {
            return Ok(());
        }

        self_.texture = new_value;

        let new_width: f32 = self_
            .texture
            .getattr(py, intern!(py, "width"))
            .unwrap()
            .extract(py)
            .expect("Failed to Load Width from Texture");
        self_.width = new_width * self_.scale.0;

        let new_height: f32 = self_
            .texture
            .getattr(py, intern!(py, "height"))
            .unwrap()
            .extract(py)
            .expect("Failed to Load Height From Texture");
        self_.height = new_height * self_.scale.1;

        for sprite_list in self_.sprite_lists.iter() {
            sprite_list.call_method1(py, intern!(py, "_update_texture"), (&self_,))?;
        }

        Ok(())
    }

    #[getter]
    fn get_color(&self) -> PyResult<(u8, u8, u8, u8)> {
        Ok(self.color)
    }

    #[setter]
    fn set_color(mut self_: PyRefMut<BasicSprite>, py: Python<'_>, new_value: Vec<u8>) -> PyResult<()> {
        let new_color: (u8, u8, u8, u8) = match new_value.len() {
            4 => (new_value[0], new_value[1], new_value[2], new_value[3]),
            3 => (new_value[0], new_value[1], new_value[2], self_.color.3),
            _ => panic!("Color must be 3 or 4 ints from 0-255"),
        };

        if new_color == self_.color {
            return Ok(());
        }

        self_.color = new_color;

        for sprite_list in self_.sprite_lists.iter() {
            sprite_list.call_method1(py, intern!(py, "_update_color"), (&self_,))?;
        }

        Ok(())
    }

    #[getter]
    fn get_alpha(&self) -> PyResult<u8> {
        Ok(self.color.3)
    }

    #[setter]
    fn set_alpha(mut self_: PyRefMut<BasicSprite>, py: Python<'_>, new_value: u8) -> PyResult<()> {
        if self_.color.3 == new_value {
            return Ok(());
        }

        self_.color.3 = new_value;

        for sprite_list in self_.sprite_lists.iter() {
            sprite_list.call_method1(py, intern!(py, "_update_color"), (&self_,))?;
        }

        Ok(())
    }

    #[getter]
    fn get_visible(&self) -> PyResult<bool> {
        Ok(self.color.3 > 0)
    }

    #[setter]
    fn set_visible(mut self_: PyRefMut<BasicSprite>, py: Python<'_>, new_value: bool) -> PyResult<()> {
        self_.color.3 = if new_value { 255 } else { 0 };

        for sprite_list in self_.sprite_lists.iter() {
            sprite_list.call_method1(py, intern!(py, "_update_color"), (&self_,))?;
        }

        Ok(())
    }

    #[getter]
    fn get_width(&self) -> PyResult<f32> {
        Ok(self.width)
    }

    #[setter]
    fn set_width(mut self_: PyRefMut<BasicSprite>, py: Python<'_>, new_value: f32) -> PyResult<()> {
        if self_.width == new_value {
            return Ok(());
        }

        let tex_width: f32 = self_
            .texture
            .getattr(py, intern!(py, "width"))
            .unwrap()
            .extract(py)
            .expect("Failed to Load Width From Texture");

        self_.scale = (new_value / tex_width, self_.scale.1);
        self_.width = new_value;
        self_.hitbox.scale = self_.scale;

        for sprite_list in self_.sprite_lists.iter() {
            sprite_list.call_method1(py, intern!(py, "_update_width"), (&self_,))?;
        }

        Ok(())
    }

    #[getter]
    fn get_height(&self) -> PyResult<f32> {
        Ok(self.height)
    }

    #[setter]
    fn set_height(mut self_: PyRefMut<BasicSprite>, py: Python<'_>, new_value: f32) -> PyResult<()> {
        if self_.height == new_value {
            return Ok(());
        }

        let tex_height: f32 = self_
            .texture
            .getattr(py, intern!(py, "height"))
            .unwrap()
            .extract(py)
            .expect("Failed to Load Height From Texture");

        self_.scale = (self_.scale.0, new_value / tex_height);
        self_.height = new_value;
        self_.hitbox.scale = self_.scale;

        for sprite_list in self_.sprite_lists.iter() {
            sprite_list.call_method1(py, intern!(py, "_update_height"), (&self_,))?;
        }

        Ok(())
    }

    #[getter]
    fn get_center_x(&self) -> PyResult<f32> {
        Ok(self.position.0)
    }

    #[setter]
    fn set_center_x(self_: PyRefMut<BasicSprite>, py: Python<'_>, new_value: f32) -> PyResult<()> {
        let y = self_.position.1;
        BasicSprite::set_position(self_, py, (new_value, y))
    }

    #[getter]
    fn get_center_y(&self) -> PyResult<f32> {
        Ok(self.position.1)
    }

    #[setter]
    fn set_center_y(self_: PyRefMut<BasicSprite>, py: Python<'_>, new_value: f32) -> PyResult<()> {
        let x = self_.position.0;
        BasicSprite::set_position(self_, py, (x, new_value))
    }

    #[getter]
    fn get_left(&self) -> PyResult<f32> {
        Ok(self.hitbox.left_native())
    }

    #[setter]
    fn set_left(self_: PyRefMut<BasicSprite>, py: Python<'_>, new_value: f32) -> PyResult<()> {
        let s = &*self_;
        let leftmost = s.hitbox.left_native();
        let diff = new_value - leftmost;
        let x = s.position.0;
        BasicSprite::set_center_x(self_, py, x + diff)
    }

    #[getter]
    fn get_right(&self) -> PyResult<f32> {
        Ok(self.hitbox.right_native())
    }

    #[setter]
    fn set_right(self_: PyRefMut<BasicSprite>, py: Python<'_>, new_value: f32) -> PyResult<()> {
        let rightmost = self_.hitbox.right_native();
        let diff = rightmost - new_value;
        let x = self_.position.0;
        BasicSprite::set_center_x(self_, py, x - diff)
    }

    #[getter]
    fn get_bottom(&self) -> PyResult<f32> {
        Ok(self.hitbox.bottom_native())
    }

    #[setter]
    fn set_bottom(self_: PyRefMut<BasicSprite>, py: Python<'_>, new_value: f32) -> PyResult<()> {
        let lowest = self_.hitbox.bottom_native();
        let diff = lowest - new_value;
        let y = self_.position.1;
        BasicSprite::set_center_y(self_, py, y - diff)
    }

    #[getter]
    fn get_top(&self) -> PyResult<f32> {
        Ok(self.hitbox.top_native())
    }

    #[setter]
    fn set_top(self_: PyRefMut<BasicSprite>, py: Python<'_>, new_value: f32) -> PyResult<()> {
        let highest = self_.hitbox.top_native();
        let diff = highest - new_value;
        let y = self_.position.1;
        BasicSprite::set_center_y(self_, py, y - diff)
    }

    fn update_spatial_hash(self_: PyRef<BasicSprite>, py: Python<'_>) -> PyResult<()> {
        for sprite_list in self_.sprite_lists.iter() {
            let spatial_hash: PyObject = sprite_list
                .getattr(py, intern!(py, "spatial_hash"))
                .unwrap()
                .extract(py)
                .unwrap();

            if spatial_hash.is(&py.None()) {
                return Ok(());
            }

            spatial_hash.call_method1(py, intern!(py, "move"), (&self_,))?;
        }

        Ok(())
    }

    fn register_sprite_list(&mut self, new_list: PyObject) {
        self.sprite_lists.push(new_list);
    }

    fn remove_from_sprite_lists(mut self_: PyRefMut<BasicSprite>, py: Python<'_>) -> PyResult<()> {
        while !self_.sprite_lists.is_empty() {
            self_.sprite_lists[0].call_method1(py, intern!(py, "remove"), (&self_,))?;
        }

        self_.sprite_lists.clear();

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
                    .getattr(intern!(py, "load_texture"))
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

    #[getter]
    fn get_angle(self_: PyRef<'_, Self>) -> PyResult<f32> {
        Ok(self_.into_super().angle)
    }

    #[setter]
    fn set_angle(mut self_: PyRefMut<'_, Self>, py: Python<'_>, new_value: f32) -> PyResult<()> {
        let super_ = self_.as_mut();

        if super_.angle == new_value {
            return Ok(());
        }

        super_.angle = new_value;
        super_.hitbox.angle = new_value;

        let sprite_lists = super_.sprite_lists.clone();
        for sprite_list in sprite_lists.iter() {
            sprite_list.call_method1(py, intern!(py, "_update_height"), (&self_,))?;
        }

        Ok(())
    }
}
