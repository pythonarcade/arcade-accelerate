import random
from pathlib import Path

import arcade

from benchmark import OUT_DIR
from benchmark.tests.base import ArcadePerfTest

SPRITE_SCALING_COIN = 0.09
SPRITE_SCALING_PLAYER = 0.5
SPRITE_NATIVE_SIZE = 128
SPRITE_SIZE = int(SPRITE_NATIVE_SIZE * SPRITE_SCALING_COIN)
SCREEN_WIDTH = 1800
SCREEN_HEIGHT = 1000
SCREEN_TITLE = "Moving Sprite Stress Test - Arcade"
USE_SPATIAL_HASHING = True
DEFAULT_METHOD = 3


class Test(ArcadePerfTest):
    name = "collision"
    instances = (({"method": 3}, "Simple"),)

    def __init__(self, session_dir: Path, method: int = DEFAULT_METHOD):
        super().__init__(
            session_dir,
            size=(SCREEN_WIDTH, SCREEN_HEIGHT),
            title=SCREEN_TITLE,
            start_count=0,
            increment_count=1000,
            duration=60.0,
        )
        self.method = method
        self.name = f"collision-{self.method}"
        self.series_name = "ArcadeTest"

        # Variables that will hold sprite lists
        self.coin_list = None
        self.player_list = None
        self.player = None

    def setup(self):
        self.window.background_color = arcade.color.AMAZON
        self.coin_texture = arcade.load_texture(":resources:images/items/coinGold.png")
        # Sprite lists
        self.coin_list = arcade.SpriteList(use_spatial_hash=USE_SPATIAL_HASHING)
        self.player_list = arcade.SpriteList()
        self.player = arcade.Sprite(
            ":resources:images/animated_characters/female_person/femalePerson_idle.png",
            scale=SPRITE_SCALING_PLAYER,
        )
        self.player.center_x = random.randrange(SCREEN_WIDTH)
        self.player.center_y = random.randrange(SCREEN_HEIGHT)
        self.player.change_x = 3
        self.player.change_y = 5
        self.player_list.append(self.player)

    def add_coins(self, amount):
        """Add a new set of coins"""
        for i in range(amount):
            coin = arcade.Sprite(
                self.coin_texture,
                center_x=random.randrange(SPRITE_SIZE, SCREEN_WIDTH - SPRITE_SIZE),
                center_y=random.randrange(SPRITE_SIZE, SCREEN_HEIGHT - SPRITE_SIZE),
                scale=SPRITE_SCALING_COIN,
            )
            self.coin_list.append(coin)

    def on_draw(self):
        super().on_draw()
        self.coin_list.draw()
        self.player_list.draw()

    def on_update(self, delta_time: float):
        super().on_update(delta_time)

        self.player_list.update()
        if self.player.center_x < 0 and self.player.change_x < 0:
            self.player.change_x *= -1
        if self.player.center_y < 0 and self.player.change_y < 0:
            self.player.change_y *= -1

        if self.player.center_x > SCREEN_WIDTH and self.player.change_x > 0:
            self.player.change_x *= -1
        if self.player.center_y > SCREEN_HEIGHT and self.player.change_y > 0:
            self.player.change_y *= -1

        coin_hit_list = arcade.check_for_collision_with_list(
            self.player, self.coin_list, method=self.method
        )
        for coin in coin_hit_list:
            coin.center_x = random.randrange(SCREEN_WIDTH)
            coin.center_y = random.randrange(SCREEN_HEIGHT)

    def update_state(self):
        # Figure out if we need more coins
        if self.timing.target_n > len(self.coin_list):
            new_coin_amount = self.timing.target_n - len(self.coin_list)
            self.add_coins(new_coin_amount)
        self.coin_list.write_sprite_buffers_to_gpu()


def run():
    Test().run(OUT_DIR)
