import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BastionlistComponent } from './bastionlist.component';

describe('BastionlistComponent', () => {
  let component: BastionlistComponent;
  let fixture: ComponentFixture<BastionlistComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ BastionlistComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(BastionlistComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
